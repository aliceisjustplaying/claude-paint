eval(File.read(File.join(__dir__,'root.rb')).split("n='views';")[0])
def kill(n);pid=File.read(ROOT+"/out/easel/#{n}/lock").to_i;Process.kill('TERM',pid);sleep 0.2;end
# Empty setup request lifecycle: all faults at whole submitted chunk boundary.
n='setup';ok(n,'open',n);c=cpu(n);a=req(n,'do',SLOW+'blocker=true');ob=observe(n,a,c);b=req(n,'do',SETUP);b.close;a.read;a.close;rec('setup-queued-abort',{observation:ob,status:cli(n,'status')})
c=cpu(n);a=req(n,'do',SETUP+';'+SLOW+'setupfinished=true');ob=observe(n,a,c);b=req(n,'do',SETUP.sub('size=100','size=200').sub('aspect=5','aspect=2').sub('linen=12','linen=20'));pending=!IO.select([b],nil,nil,0);a.close;br=b.read;b.close;rec('setup-running-disconnect',{observation:ob,second_pending:pending,second:br,status:cli(n,'status'),check:cli(n,'do','assert(setupfinished);print(W,H)')});ok(n,'close')
%w[kill external persistence file].each do |kind|
 n='setup'+kind;ok(n,'open',n);log=ROOT+"/paintings/lua/#{n}.lua";w=ROOT+"/out/easel/#{n}/committed.lua";original=File.binread(log)
 if kind=='file'
  f=ROOT+'/setup.lua';File.write(f,SETUP+';'+SLOW+'filesetup=true');c=cpu(n);t=Thread.new{cli(n,'do','-f',f)};sleep 0.2;active=t.alive?;advanced=cpu(n)!=c;File.write(f,SETUP.sub('size=100','size=200'));rec('setup-file',{pending:active,cpu_advanced:advanced,reply:t.value,status:cli(n,'status')});ok(n,'close');next
 end
 if kind=='persistence'
  FileUtils.mkdir_p(File.dirname(w)+'/committed.pending');rec('setup-persistence',{reply:cli(n,'do',SETUP),status:cli(n,'status'),diverged:File.binread(log)!=File.binread(w)});kill(n);FileUtils.rmdir(File.dirname(w)+'/committed.pending');File.binwrite(log,original);next
 end
 if kind=='external'
  File.write(log,'-- before');rec('setup-external-before',{reply:cli(n,'do',SETUP)});File.binwrite(log,original)
 end
 c=cpu(n);a=req(n,'do',SETUP+';'+SLOW+'uncommitted=true');ob=observe(n,a,c)
 if kind=='kill';kill(n);else File.write(log,'-- during');end
 reply=a.read;a.close;restricted=cli(n,'status');kill(n) unless kind=='kill';File.binwrite(log,original);rec('setup-'+kind,{observation:ob,reply:reply,restricted:restricted,reopen:cli(n,'open',n),state:cli(n,'status')});ok(n,'close')
end
# A lost successful reply followed by retry really repeats work.
n='retry';ok(n,'open',n);ok(n,'do','counter=0');c=cpu(n);a=req(n,'do',SLOW+'counter=counter+1');ob=observe(n,a,c);a.close;first=ok(n,'do','print(counter)');ok(n,'do','counter=counter+1');rec('retry',{observation:ob,first:first,second:ok(n,'do','print(counter)')});ok(n,'close')
# Closure saves remaining files even if checkpoint temporary destination is a directory.
n='closefault';ok(n,'open',n);ok(n,'do',SETUP);d=ROOT+"/out/easel/#{n}";FileUtils.mkdir_p(d+'/live.ckpt.part');reply=cli(n,'close');rec('close-checkpoint-fault',{reply:reply,files:Dir.children(d),log_exists:File.file?(ROOT+"/paintings/lua/#{n}.lua")});FileUtils.rmdir(d+'/live.ckpt.part');rec('log-only-reopen',{reply:cli(n,'open',n),status:cli(n,'status')});ok(n,'close');FileUtils.rm_f(d+'/live.ckpt');rec('deleted-checkpoint',{reply:cli(n,'open',n),status:cli(n,'status')});ok(n,'close')
# Dead socket and denied root.
n='stale';d=ROOT+"/out/easel/#{n}";FileUtils.mkdir_p(d);sock=UNIXServer.new(d+'/sock');sock.close;rec('stale-socket',{before:cli(n,'status'),open:cli(n,'open',n),after:cli(n,'status')});ok(n,'close')
denied=ROOT+'/denied';FileUtils.mkdir_p(denied);File.chmod(0555,denied);o,e,s=Open3.capture3({'EASEL_ROOT'=>denied,'EASEL_SESSION'=>'denied'},BIN,'open','denied');File.chmod(0755,denied);rec('denied-root',{out:o,err:e,exit:s.exitstatus})
