require 'open3';require 'fileutils';require 'json'
bin,root,out,source=ARGV;raise 'source worktree is fourth argument' unless source;FileUtils.mkdir_p(root+'/bin');FileUtils.cp(bin,root+'/bin/easel');exe=root+'/bin/easel'
def cli(exe,*a);o,e,s=Open3.capture3(exe,*a);raise o+e unless s.success?;o;end
cli(exe,'open');cli(exe,'do','canvas{size=100,aspect=0.2,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}};p=pile{{"red earth",1}};b=brush("round",20);b:load(p);b:touch(100,80)')
hi,ho,he,ht=Open3.popen3('node',File.join(__dir__,'channel-concurrent.mjs'),root,source);raise 'not ready' unless ho.gets.strip=='ready'
pid=File.read(root+'/out/easel/painting/lock').to_i;cpu=-> {Open3.capture3('/bin/ps','-p',pid.to_s,'-o','time=')[0]};before=cpu.call
i,o,e,t=Open3.popen3(exe,'look','--mode','value','--size','2000');i.close;observed=nil
100.times do
 after=cpu.call;if before!=after && !IO.select([o],nil,nil,0);observed={before:before,after:after,cli_output_pending:true};break;end;sleep 0.001
end
raise 'no pending CLI render observed' unless observed
hi.puts('go');hi.flush;hi.close;invoked=ho.gets.strip;overlap=t.alive? && !IO.select([o],nil,nil,0);first=o.read;err=e.read;ec=t.value.exitstatus;second=ho.read;herr=he.read;hc=ht.value.exitstatus;ho.close;he.close;o.close;e.close
File.write(out,JSON.pretty_generate({observation:observed,helper_invoked:invoked,cli_still_pending_when_helper_started:overlap,cli:{reply:first,error:err,exit:ec},harness_client:{reply:second,error:herr,exit:hc}}).gsub(root,'<studio>'))
cli(exe,'close')
