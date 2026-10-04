require 'open3';require 'json';require 'fileutils';require 'time';require 'timeout'
BIN,ROOT,OUT=ARGV;raise "binary, disposable root and output required" unless OUT
FileUtils.mkdir_p(ROOT);M=Mutex.new;$data={started:Time.now.utc.iso8601,source_commit:'4e525e50897807e9b5f734071dfeb330f1a393d3',cases:{}}
def update(n,v);M.synchronize{$data[:cases][n]=v;File.write(OUT,JSON.pretty_generate($data))};end
def run(n,*args);o,e,s=Open3.capture3({'EASEL_ROOT'=>ROOT,'EASEL_SESSION'=>n},BIN,*args);raise(o+e) unless s.success?;o;end
slow='local sum=0;for i=1,200000000 do sum=sum+i end;assert(sum>0)'
%w[stalled progressing].each do |n|
 run(n,'open',n)
 (n=='stalled' ? 1 : 19).times{run(n,'do',slow)}
 run(n,'close')
end
threads=%w[stalled progressing].map do |name|
 Thread.new do
  record={start:Time.now.utc.iso8601,pauses:[],status:'running'};update(name,record.dup)
  t=Process.clock_gettime(Process::CLOCK_MONOTONIC);server=nil;stopped=false
  Open3.popen3({'EASEL_ROOT'=>ROOT,'EASEL_SESSION'=>name},BIN,'open',name) do |i,o,e,w|
   i.close;errors=Thread.new{e.read};output=Thread.new{o.read};dir=ROOT+'/out/easel/'+name
   begin
    deadline=t+2200;nextchunk=1;pause_until=nil
    loop do
     now=Process.clock_gettime(Process::CLOCK_MONOTONIC);raise 'timer fixture exceeded bound' if now>deadline
     break unless w.alive?
     text=File.read(dir+'/server.log') rescue ''
     server=(File.read(dir+'/lock').to_i rescue 0)
     if !stopped && text.include?("resuming chunk #{nextchunk}/") && server>0
      Process.kill('STOP',server);stopped=true
      record[:pauses]<<{chunk:nextchunk,at_seconds:(now-t).round(3)};record[:status]='server paused';update(name,Marshal.load(Marshal.dump(record)))
      pause_until=name=='stalled' ? nil : now+105
     elsif stopped && pause_until && now>=pause_until
      Process.kill('CONT',server);stopped=false;nextchunk+=1;pause_until=nil
      # Last pause is also resumed: nineteen real progress resets span >30min.
     end
     sleep 0.01
    end
    record[:seconds]=(Process.clock_gettime(Process::CLOCK_MONOTONIC)-t).round(3);record[:exit]=w.value.exitstatus;record[:stdout]=output.value;record[:stderr]=errors.value;record[:status]='complete'
   ensure
    if server && server>0
     Process.kill('CONT',server) rescue nil
     if name=='stalled';Process.kill('TERM',server) rescue nil;else;run(name,'close') rescue nil;end
    end
   end
  end
  update(name,record)
 rescue =>ex
  record||={};record[:error]=ex.message;record[:status]='fixture error';update(name,record)
 end
end
threads.each(&:join)
