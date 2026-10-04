# One-off verification of actual easel request/lifecycle boundaries.
# Usage: ruby transport.rb /path/to/default-easel /short/disposable/root /path/to/results.json
require 'open3';require 'socket';require 'json';require 'fileutils';require 'digest';require 'timeout'
BIN,ROOT,OUT=ARGV;raise 'three arguments required' unless OUT
FileUtils.mkdir_p(ROOT);$records=[]
def record(id,data)
 $records<<{id:id}.merge(data);File.write(OUT,JSON.pretty_generate($records).gsub(ROOT,'<isolated-root>').gsub(BIN,'<easel>'));puts id
end
def cli(name,*args)
 o,e,s=Open3.capture3({'EASEL_ROOT'=>ROOT,'EASEL_SESSION'=>name,'EASEL_BOX'=>nil},BIN,*args);{stdout:o,stderr:e,exit:s.exitstatus}
end
def ok(name,*args);r=cli(name,*args);raise r.inspect unless r[:exit]==0;r[:stdout];end
def request(name,src,head='do')
 s=UNIXSocket.new(ROOT+'/out/easel/'+name+'/sock');body=head+"\n"+src;s.write("#{body.bytesize}\n#{body}");s
end
def cpu(name)
 pid=File.read(ROOT+'/out/easel/'+name+'/lock').to_i
 o,e,s=Open3.capture3('/bin/ps','-p',pid.to_s,'-o','time=');raise e unless s.success?;o.strip
end
def observe(name,s,before)
 deadline=Process.clock_gettime(Process::CLOCK_MONOTONIC)+5
 loop do
  after=cpu(name)
  return {before:before,after:after,reply_pending:true} if after!=before && IO.select([s],nil,nil,0).nil?
  raise 'request finished before observing execution' if IO.select([s],nil,nil,0)
  raise 'could not observe CPU activity' if Process.clock_gettime(Process::CLOCK_MONOTONIC)>deadline
  sleep 0.025
 end
end
def kill_server(name)
 pid=File.read(ROOT+'/out/easel/'+name+'/lock').to_i;raise 'invalid owned pid' unless pid>1
 Process.kill('TERM',pid);50.times{break unless File.exist?("/proc/#{pid}") || system('/bin/kill','-0',pid.to_s,out:File::NULL,err:File::NULL);sleep 0.02}
end
SETUP='canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; b=brush("round",4);b:load(p,0.8);h=pencil("HB");r=rag{width=20};r:dip(0.5);m=rect(100,40,80,80)'
SLOW='local sum=0;for i=1,300000000 do sum=sum+i end;assert(sum>0);'
OPS={brush:'b:touch(140,80)',mask:'m=rect(300,40,80,80)',drawing:'h:line({{100,80},{200,80}})',rag:'r:blot(140,80)',passage:'work(m,{hand="detail",pile=p,coverage=0.3,clip=true})',time:'wait(1)'}
OPS.each do |feature,op|
 name=feature.to_s;ok(name,'open',name);ok(name,'do',SETUP)
 begin
  # A queued closed connection never reaches the operation.
  a=request(name,SLOW+'blocker=true');baseline=cpu(name);obs=observe(name,a,baseline)
  b=request(name,op+';queued_marker=true');b.close;reply=a.read;a.close
  check=ok(name,'do','assert(queued_marker==nil and blocker==true)')
  record("#{feature}-queued-abort",{observation:obs,first_reply:reply,check:check,result:'pass'})
  # The operation executes within a pending chunk; client loss is not server abort.
  baseline=cpu(name);a=request(name,op+';'+SLOW+'running_marker=true');obs=observe(name,a,baseline);a.close
  check=ok(name,'do','assert(running_marker==true)')
  record("#{feature}-running-disconnect",{observation:obs,check:check,result:'pass'})
  # Two real connections submit different chunks and serialize state.
  baseline=cpu(name);a=request(name,op+';'+SLOW+'serial_marker=17');obs=observe(name,a,baseline)
  b=request(name,'assert(serial_marker==17);'+op+';serial_marker=18')
  pending=IO.select([b],nil,nil,0).nil?;ra=a.read;rb=b.read;a.close;b.close
  raise 'serialization failed' unless pending && ra.start_with?("ok\n") && rb.start_with?("ok\n")
  record("#{feature}-serialization",{observation:obs,second_pending:pending,first:ra,second:rb,result:'pass'})
  # Real file-input client reads the complete source before server execution.
  file=ROOT+"/#{name}-input.lua";File.write(file,op+';'+SLOW+'file_marker="captured"')
  baseline=cpu(name);thread=Thread.new{cli(name,'do','-f',file)}
  sleep 0.2;active=thread.alive?;changed_cpu=cpu(name)!=baseline;File.write(file,'file_marker="replacement"')
  rr=thread.value;check=ok(name,'do','assert(file_marker=="captured")')
  raise 'input not observed pending' unless active && changed_cpu && rr[:exit]==0
  record("#{feature}-file-capture",{pending_when_file_changed:active,cpu_advanced:changed_cpu,reply:rr,check:check,result:'pass'})
  # Edits before submission are refused and identical restoration recovers.
  log=ROOT+"/paintings/lua/#{name}.lua";witness=ROOT+"/out/easel/#{name}/committed.lua";original=File.binread(log)
  File.write(log,"-- outside edit\n");rr=cli(name,'do',op);File.binwrite(log,original)
  raise 'edit not refused' unless rr[:exit]!=0 && rr[:stderr].include?('integrity')
  record("#{feature}-external-before",{reply:rr,result:'pass'})
  # Edit after execution began, before persistence. Old witness remains authoritative.
  baseline=cpu(name);a=request(name,op+';'+SLOW+'external_marker=true');obs=observe(name,a,baseline)
  File.write(log,"-- during edit\n");ra=a.read;a.close;restricted=cli(name,'status')
  raise 'concurrent edit not refused' unless ra.start_with?("err\n") && ra.include?('integrity') && restricted[:exit]!=0
  kill_server(name);File.binwrite(log,original);ok(name,'open',name);check=ok(name,'do','assert(external_marker==nil)')
  record("#{feature}-external-during",{observation:obs,reply:ra,restricted:restricted,baseline_reopen:check,result:'pass'})
  # Kill after a real material operation in an uncommitted chunk, then resume log.
  original=File.binread(log);ok(name,'save',ROOT+"/#{name}-before.png");baseline=cpu(name)
  a=request(name,op+';'+SLOW+'killed_marker=true');obs=observe(name,a,baseline);kill_server(name);ra=(a.read rescue $!.message);a.close
  ok(name,'open',name);check=ok(name,'do','assert(killed_marker==nil)');ok(name,'save',ROOT+"/#{name}-after.png")
  same=Digest::SHA256.file(ROOT+"/#{name}-before.png").hexdigest==Digest::SHA256.file(ROOT+"/#{name}-after.png").hexdigest
  raise 'killed chunk changed replay image' unless same
  record("#{feature}-server-kill",{observation:obs,reply:ra,baseline_reopen:check,png_equal:same,result:'pass'})
  # Material operation returns, then the witness write fails after log append.
  original=File.binread(log);oldw=File.binread(witness);FileUtils.mkdir_p(File.dirname(witness)+'/committed.pending')
  rr=cli(name,'do',op+';persist_marker=true');diverged=File.binread(log)!=File.binread(witness);restricted=cli(name,'close')
  raise 'persistence fault not established' unless rr[:exit]!=0 && diverged && restricted[:exit]!=0
  record("#{feature}-persistence",{reply:rr,log_witness_diverge:diverged,close:restricted,result:'pass'})
  kill_server(name);FileUtils.rmdir(File.dirname(witness)+'/committed.pending');File.binwrite(log,original);File.binwrite(witness,oldw);ok(name,'open',name)
 ensure
  rr=cli(name,'close');record("#{feature}-cleanup",rr)
 end
end
