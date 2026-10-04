eval(File.read(File.join(__dir__,'root.rb')).split("n='views';")[0])
def custom(bin,root,env,*args);o,e,s=Open3.capture3({'EASEL_ROOT'=>root,'EASEL_SESSION'=>nil,'EASEL_BOX'=>nil}.merge(env),bin,*args);{args:args,out:o,err:e,exit:s.exitstatus};end
pbin=ROOT+'/p/bin/easel';FileUtils.mkdir_p(File.dirname(pbin));FileUtils.cp(File.dirname(BIN)+'/easel-painter',pbin)
r=[];[['open','named'],['-s','elsewhere','status'],['open'],['do',SETUP],['status'],['run','x.lua'],['check'],['close']].each{|a|r<<custom(pbin,ROOT+'/ignored',{'EASEL_SESSION'=>'elsewhere'},*a)};rec('painter',{calls:r,logs:Dir[ROOT+'/p/paintings/lua/*']})
# Independent roots, a selected caller cannot move another pending request.
n='a';ok(n,'open',n);ok(n,'do','rootmarker="A"');other=ROOT+'/other';r=custom(BIN,other,{},'open','a');r2=custom(BIN,other,{},'do','rootmarker="B"');rec('roots',{open:r,do:r2,a:cli(n,'globals'),b:custom(BIN,other,{},'globals')});custom(BIN,other,{},'close')
ok('b','open','b');ok('b','do','bmarker=true');c=cpu(n);a=req(n,'do',SLOW+'amarker=true');obs=observe(n,a,c);second=cli('b','do','bsecond=true');first=a.read;a.close;rec('selected-during',{observation:obs,first:first,second:second,aglobals:cli(n,'globals'),bglobals:cli('b','globals'),changed_environment:custom(BIN,ROOT,{'EASEL_SESSION'=>'a','EASEL_BOX'=>'invalid'},'status')});ok('b','close');ok(n,'close')
# New box configuration and recorded-box conflict are explicit errors.
bin=ROOT+'/boxbin/easel';FileUtils.mkdir_p(File.dirname(bin));FileUtils.cp(BIN,bin);bf=File.dirname(bin)+'/box';br=ROOT+'/boxroot';calls=[]
calls<<custom(bin,br,{},'open','default');calls<<custom(bin,br,{},'close');calls<<custom(bin,br,{'EASEL_BOX'=>'sargent'},'open','env');calls<<custom(bin,br,{'EASEL_BOX'=>'sargent'},'do','print(type(canvas))');calls<<custom(bin,br,{},'close');File.write(bf,"sargent\n");calls<<custom(bin,br,{},'open','file');calls<<custom(bin,br,{},'close');calls<<custom(bin,br,{'EASEL_BOX'=>'tube box'},'open','conflict');File.write(bf,"tube box\n");calls<<custom(bin,br,{},'open','env');FileUtils.rm_f(bf);calls<<custom(bin,br,{},'open','env');calls<<custom(bin,br,{},'status');calls<<custom(bin,br,{},'close');rec('boxes',{calls:calls,recorded:File.read(br+'/paintings/lua/env.lua')})
# Opening abort, second open, ordinary request and concurrent external edit during replay.
%w[abort edit].each do |kind|
 n='replay'+kind;ok(n,'open',n);ok(n,'do',SLOW+'replaymarker=true');ok(n,'close');log=ROOT+"/paintings/lua/#{n}.lua";original=File.binread(log)
 i,o,e,t=Open3.popen3({'EASEL_ROOT'=>ROOT,'EASEL_SESSION'=>n},BIN,'open',n);i.close
 ready=Time.now+5;loop{break if File.read(ROOT+"/out/easel/#{n}/server.log").include?('resuming chunk') rescue nil;raise 'replay not started' if Time.now>ready;sleep 0.01}
 if kind=='abort'
  Process.kill('TERM',t.pid);t.value;second=Thread.new{cli(n,'open',n)};ordinary=cli(n,'status');rec('open-abort-concurrent',{client_output:o.read,client_error:e.read,ordinary:ordinary,second_open:second.value,final:cli(n,'status')});ok(n,'close')
 else
  File.write(log,original+"-- outside during replay\n");reply={out:o.read,err:e.read,exit:t.value.exitstatus};state=cli(n,'status');rec('replay-edit',{opening:reply,status:state});File.binwrite(log,original);cli(n,'close')
 end
 o.close;e.close
end
n='closededit';ok(n,'open',n);ok(n,'do','marker=true');ok(n,'close');log=ROOT+"/paintings/lua/#{n}.lua";old=File.binread(log);File.write(log,old+'-- altered');rec('closed-edit',{open:cli(n,'open',n)});File.binwrite(log,old);ok(n,'open',n);ok(n,'close')
rec('width',{reply:cli('width','open','width','--width','64')})
rec('legacy-current',{open:cli('current','open','current'),reply:cli('current','do','canvas{style="friedrich",palette="friedrich_1820_greens",aspect=1.4,seed=23}'),close:cli('current','close')})
