require 'open3';require 'socket';require 'json';require 'fileutils';require 'digest'
BIN,ROOT,OUT=ARGV;FileUtils.mkdir_p([ROOT,OUT]);$r=[]
def rec(id,data);$r<<{id:id}.merge(data);File.write(OUT+'/root.json',JSON.pretty_generate($r).gsub(ROOT,'<root>').gsub(BIN,'<easel>'));puts id;end
def cli(n,*a,stdin:nil);o,e,s=Open3.capture3({'EASEL_ROOT'=>ROOT,'EASEL_SESSION'=>n,'EASEL_BOX'=>nil},BIN,*a,stdin_data:stdin.to_s);{args:a,out:o,err:e,exit:s.exitstatus};end
def ok(n,*a,**kw);r=cli(n,*a,**kw);raise r.inspect unless r[:exit]==0;r[:out];end
def req(n,head,body='');s=UNIXSocket.new(ROOT+"/out/easel/#{n}/sock");b=head.tr(" ","\t")+"\n"+body;s.write("#{b.bytesize}\n#{b}");s;end
def cpu(n);Open3.capture3('/bin/ps','-p',File.read(ROOT+"/out/easel/#{n}/lock").to_i.to_s,'-o','time=')[0];end
def observe(n,s,c);100.times{d=cpu(n);return {cpu_before:c,cpu_after:d,pending:true} if d!=c && !IO.select([s],nil,nil,0);raise 'finished early' if IO.select([s],nil,nil,0);sleep 0.01};raise 'not running';end
def hash(p);Digest::SHA256.file(p).hexdigest;end
def shot(n,label);p=OUT+'/'+label+'.png';ok(n,'save',p);p;end
def look(n,label,*opts);reply=ok(n,'look',*opts);p=reply.split(' (')[0].strip;dest=OUT+'/'+label+'.png';FileUtils.cp(p,dest);{reply:reply,path:dest,sha:hash(dest)};end
SETUP='canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}},seed=1};p=pile{{"red earth",1}};q=pile{{"cobalt blue",1}};b=brush("round",4);b:load(p,0.8);r=rag{width=20};r:dip(0.5);h=pencil("HB");m=rect(100,40,80,80);b:touch(100,80);h:line({{300,40},{350,150}})'
SLOW='local sum=0;for i=1,300000000 do sum=sum+i end;assert(sum>0);'
def attempt(id);yield;rescue=>e;rec(id,{error:e.message,backtrace:e.backtrace.first(2)});end
n='views';ok(n,'open',n)
rec('empty',{status:cli(n,'status'),log:cli(n,'log'),look:cli(n,'look'),globals:cli(n,'globals')})
ok(n,'do',SETUP)
snapshot='print(wait(0),b:fullness(),r.load,r.soaked,r.damp,r.fold)'
before=ok(n,'do',snapshot);baseline=shot(n,'baseline');lh=hash(ROOT+"/paintings/lua/#{n}.lua")
views={};{'plain'=>[],'value'=>['--mode','value'],'gray'=>['--mode','gray'],'squint'=>['--mode','squint'],'blur'=>['--mode','blur'],'mirror'=>['--mode','mirror'],'value-mirror'=>['--mode','value,mirror'],'squint-mirror'=>['--mode','squint,mirror'],'grid'=>['--grid'],'grid50'=>['--grid','50'],'mirror-grid'=>['--mode','mirror','--grid','50'],'combined-crop'=>['--crop','300,150,100,50','--mode','value,mirror','--grid','50','--size','2000'],'palette'=>['--palette'],'plain-again'=>[]}.each{|k,v|views[k]=look(n,k,*v)}
afterhash=hash(shot(n,'after-views'));same_log=lh==hash(ROOT+"/paintings/lua/#{n}.lua");after=ok(n,'do',snapshot)
rec('view-state',{views:views,before:before,after:after,png_equal:hash(baseline)==afterhash,log_unchanged:same_log,continuation:cli(n,'do','b:touch(700,100)')})
rec('invalid-views',{before:cli(n,'status'),badmode:cli(n,'look','--mode','bad'),badcrop:cli(n,'look','--crop','0,0,900,150'),after:cli(n,'status'),undo:cli(n,'undo'),restore:cli(n,'look','--restore','1')})
rec('post-look',{success:cli(n,'do','--look','b:touch(600,100)'),failure:cli(n,'do','--look','b:touch(500,100);error("abort")'),later_plain:cli(n,'look')})
attempt('postcommit-file-fault') do
 witness=ROOT+"/out/easel/#{n}/committed.lua";dir=File.dirname(witness);old=File.binread(witness)
 t=Thread.new{cli(n,'do','--look','b:touch(800,100);postlookmarker=true')}
 deadline=Time.now+5;Thread.pass while File.binread(witness)==old && Time.now<deadline
 File.chmod(0555,dir);reply=t.value;File.chmod(0755,dir)
 rec('postcommit-file-fault',{reply:reply,committed:cli(n,'do','assert(postlookmarker==true)'),freshlook:cli(n,'look')})
end
# Actual command rollback of tool fields, clock and random state.
b=ok(n,'do',snapshot);pre=hash(shot(n,'rollback-before'));log=hash(ROOT+"/paintings/lua/#{n}.lua")
r=cli(n,'do','b:wipe();r:dip(1);r:blot(100,80);wait(123);error("rollback")');unchanged=log==hash(ROOT+"/paintings/lua/#{n}.lua");a=ok(n,'do',snapshot)
rec('rollback',{before:b,failed:r,after:a,log_unchanged:unchanged,png_equal:pre==hash(shot(n,'rollback-after')),caught:cli(n,'do','local yes,err=pcall(function()b:touch(0/0,1)end);print(yes,err);caught_marker=true')})
# Journal CLI and read-only ordering.
journal=ROOT+'/notes/journal.md';pre=hash(shot(n,'journal-before'));log=hash(ROOT+"/paintings/lua/#{n}.lua")
records={status:cli(n,'status'),globals:cli(n,'globals'),log:cli(n,'log'),multiline:cli(n,'note',"first\nsecond\n\nthird"),text:File.read(journal)}
prev=File.read(journal);records[:empty]=cli(n,'note'," \n ");records[:empty_unchanged]=prev==File.read(journal);records[:stdin]=cli(n,'note','-',stdin:"stdin first\nstdin second\n");File.write(journal,'without newline');records[:no_lf]=cli(n,'note','next');records[:no_lf_text]=File.read(journal);records[:denied_before]=File.read(journal);File.chmod(0444,journal);records[:denied]=cli(n,'note','denied');File.chmod(0644,journal);records[:denied_unchanged]=records[:denied_before]==File.read(journal);records[:log_unchanged]=log==hash(ROOT+"/paintings/lua/#{n}.lua");records[:png_equal]=pre==hash(shot(n,'journal-after'));rec('journal',records)
['status','globals','log','note'].each do |head|
 c=cpu(n);s=req(n,'do',SLOW+"ordered_#{head}=42");obs=observe(n,s,c);b=req(n,head,head=='note' ? 'queued immutable note' : '');pending=!IO.select([b],nil,nil,0);first=s.read;second=b.read;s.close;b.close;rec('ordered-'+head,{observation:obs,pending:pending,first:first,second:second})
end
c=cpu(n);s=req(n,'do',SLOW+'journal_blocker=true');obs=observe(n,s,c);b=req(n,'note','must not append');b.close;s.read;s.close;ok(n,'status');rec('queued-note-abort',{observation:obs,absent:!File.read(journal).include?('must not append')})
ok(n,'do','heldtable={x=1}');g1=ok(n,'globals');ok(n,'do','heldtable.x=2');rec('global-assignment-stamp',{before:g1,after:ok(n,'globals')})
status=ok(n,'status');ok(n,'note','canvas size=999 different setup');rec('note-not-paint',{before:status,after:ok(n,'status')})
rec('missing',{status:cli('missing','status'),look:cli('missing','look'),note:cli('missing','note','lost'),log:cli('missing','log')})
ok('other','open','other');ok('other','do',SETUP.sub('red earth','cobalt blue'));ok('other','note','other session same journal');rec('session-isolation',{first:cli(n,'status'),other:cli('other','status'),firstlook:look(n,'session-first'),otherlook:look('other','session-other'),journal:File.read(journal),explicit:cli('other','-s',n,'status')});ok('other','close')
# Failed random attempt does not consume later sequence.
random=[]
%w[rand_a rand_b].each do |name|
 ok(name,'open',name);ok(name,'do',SETUP);failure=cli(name,'do','print(math.random());b:touch(math.random()*1000,80);error("random abort")') if name=='rand_a';r=ok(name,'do','print(math.random(),math.random());b:touch(math.random()*1000,80)');p=shot(name,name);random<<{name:name,failed:failure,reply:r,png:hash(p),seed_repeat:cli(name,'do','math.randomseed(77);local x=math.random();math.randomseed(77);assert(x==math.random());print(x)')};ok(name,'close')
end
rec('random-rollback',{sessions:random})
# Journal shares root but painting globals and log remain session-local.
rec('sandbox',{reply:cli(n,'do','print(type(os),type(io),type(require));os.execute("true")')})
ok(n,'close')
