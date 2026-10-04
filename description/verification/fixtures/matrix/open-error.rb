eval(File.read(File.join(__dir__,'root.rb')).split("n='views';")[0])
n='openerror';ok(n,'open',n);ok(n,'do',SLOW+'marker=true');ok(n,'close');log=ROOT+"/paintings/lua/#{n}.lua";old=File.binread(log);serverlog=ROOT+"/out/easel/#{n}/server.log"
i,o,e,t=Open3.popen3({'EASEL_ROOT'=>ROOT,'EASEL_SESSION'=>n},BIN,'open',n);i.close;start=Process.clock_gettime(Process::CLOCK_MONOTONIC)
loop{break if File.read(serverlog).include?('resuming chunk');sleep 0.01};File.write(log,old+"-- changed after replay captured\n");sleep 4
status=cli(n,'status');logtext=File.read(serverlog);running=t.alive?;duration=Process.clock_gettime(Process::CLOCK_MONOTONIC)-start;Process.kill('TERM',t.pid) if running;t.value
rec('open-ignores-integrity-error',{seconds:duration,opening_still_running:running,status:status,serverlog:logtext,client_out:o.read,client_err:e.read,terminated_by_fixture:running});o.close;e.close;File.binwrite(log,old);ok(n,'close')
