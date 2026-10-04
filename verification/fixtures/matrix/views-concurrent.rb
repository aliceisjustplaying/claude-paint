eval(File.read(File.join(__dir__,'root.rb')).split("n='views';")[0])
n='render';ok(n,'open',n);ok(n,'do',SETUP.sub('aspect=5','aspect=1'))
[['look --mode value --size 2000','look --mode mirror --size 2000'],['look --size 2000','do']].each_with_index do |(ha,hb),i|
 attempt("render-concurrent#{i}") do
 c=cpu(n);a=req(n,ha);obs=observe(n,a,c);b=req(n,hb,hb=='do' ? 'b:touch(700,100)' : '');pending=!IO.select([b],nil,nil,0);ra=a.read;rb=b.read;a.close;b.close;rec("render-concurrent#{i}",{observation:obs,second_pending:pending,first:ra,second:rb,later:look(n,"concurrent#{i}",'--size','2000')});[ra,rb].each_with_index{|r,j|path=r.lines[1]&.split(' (')&.first&.strip;FileUtils.cp(path,OUT+"/concurrent#{i}-#{j}.png") if path && File.file?(path)}
 end
end
attempt('look-disconnect') do
 c=cpu(n);a=req(n,'look --size 2000');obs=observe(n,a,c);a.close;rec('look-disconnect',{observation:obs,status:cli(n,'status'),look:cli(n,'look','--size','80')})
end
c=cpu(n);a=req(n,'do',SLOW+'blocker=true');obs=observe(n,a,c);before=Dir[ROOT+"/out/easel/#{n}/look-*.png"];b=req(n,'look');b.close;a.read;a.close;ok(n,'status');rec('queued-look-disconnect',{observation:obs,no_new_images:before==Dir[ROOT+"/out/easel/#{n}/look-*.png"]})
log=ROOT+"/paintings/lua/#{n}.lua";original=File.binread(log);File.write(log,original+"-- external\n");rec('look-integrity-before',{look:cli(n,'look'),paint:cli(n,'do','marker=true')});File.binwrite(log,original)
attempt('look-integrity-during') do
 c=cpu(n);a=req(n,'look --size 2000');obs=observe(n,a,c);File.write(log,original+"-- concurrent edit\n");reply=a.read;a.close;rec('look-integrity-during',{observation:obs,reply:reply,next:cli(n,'status')});File.binwrite(log,original)
end
attempt('kill-render') do
 c=cpu(n);a=req(n,'look --size 2000');obs=observe(n,a,c);pid=File.read(ROOT+"/out/easel/#{n}/lock").to_i;Process.kill('TERM',pid);reply=a.read;a.close;sleep 0.2;rec('kill-render',{observation:obs,reply:reply,log_equal:File.binread(log)==original,reopen:cli(n,'open',n)})
end
ok(n,'close')
