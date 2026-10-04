eval(File.read(File.join(__dir__,'root.rb')).split("n='views';")[0])
n='variants';ok(n,'open',n);ok(n,'do',SETUP.sub('aspect=5','aspect=0.2')+';for i=1,25 do _G["palette"..i]=pile{{"red earth",i},{"lead white",26-i},medium=i/30} end')
variants={'crop'=>['--crop','0,0,500,500'],'squint'=>['--mode','squint'],'mirror'=>['--mode','mirror'],'combined'=>['--mode','value,mirror'],'grid'=>['--grid','50'],'size'=>['--size','500'],'palette'=>['--palette']}
variants.each do |id,opts|
 attempt(id) do
  baseline=look(n,id+'-baseline',*opts);c=cpu(n);a=req(n,['look',*opts].join("\t"));obs=observe(n,a,c);b=req(n,"look\t--mode\tvalue\t--size\t500");pending=!IO.select([b],nil,nil,0);ra=a.read;rb=b.read;a.close;b.close;path=ra.lines[1].split(' (')[0].strip;same=hash(path)==baseline[:sha];rec(id,{options:opts,observation:obs,second_pending:pending,first:ra,second:rb,same_as_options_baseline:same})
 end
end
# Actual CLI pipe, not only socket: no output while accepted render is pending.
i,o,e,t=Open3.popen3({'EASEL_ROOT'=>ROOT,'EASEL_SESSION'=>n},BIN,'look','--mode','squint','--size','2000');i.close;began=Process.clock_gettime(Process::CLOCK_MONOTONIC);c=cpu(n);ob=observe(n,o,c);empty=!IO.select([o],nil,nil,0);first_at=Process.clock_gettime(Process::CLOCK_MONOTONIC);reply=o.read;finished=Process.clock_gettime(Process::CLOCK_MONOTONIC);err=e.read;exitcode=t.value.exitstatus;o.close;e.close;rec('cli-stdout',{observation:ob,no_bytes_while_render_pending:empty,pending_at_seconds:first_at-began,completion_seconds:finished-began,stdout:reply,stderr:err,exit:exitcode});ok(n,'close')
# Actual painter binary with a second selection attempt while its server computes.
pbin=ROOT+'/p/bin/easel';FileUtils.mkdir_p(File.dirname(pbin));FileUtils.cp(File.dirname(BIN)+'/easel-painter',pbin)
def pc(bin,*a);o,e,s=Open3.capture3({'EASEL_SESSION'=>'elsewhere'},bin,*a);{out:o,err:e,exit:s.exitstatus};end
open=pc(pbin,'open');pc(pbin,'do','marker=0');t=Thread.new{pc(pbin,'do',SLOW+'marker=1')};sleep 0.2;pending=t.alive?;selection=pc(pbin,'-s','another','status');named=pc(pbin,'open','another');rec('painter-selection-during',{opening:open,pending:pending,selection:selection,named:named,original:t.value,final:pc(pbin,'do','print(marker)'),close:pc(pbin,'close')})
