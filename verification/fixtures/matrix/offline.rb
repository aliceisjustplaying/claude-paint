require 'open3';require 'json';require 'fileutils';require 'socket'
bin,root,out=ARGV;FileUtils.mkdir_p(root);profile='(version 1)(allow default)(deny network-outbound (remote ip "*:*"))';r=[]
def execute(profile,env,*args);o,e,s=Open3.capture3(env,'/usr/bin/sandbox-exec','-p',profile,*args);{args:args,out:o,err:e,exit:s.exitstatus};end
# Loopback TCP control independently demonstrates that the sandbox denies IP networking.
server=TCPServer.new('127.0.0.1',0);port=server.addr[1];t=Thread.new{begin;c=server.accept;c.write("HTTP/1.0 200 OK\r\nContent-Length: 2\r\n\r\nok");c.close;rescue IOError;end};o,e,s=Open3.capture3('/usr/bin/curl','--noproxy','*','--max-time','2',"http://127.0.0.1:#{port}");r<<{control:{out:o,err:e,exit:s.exitstatus}};t.join
r<<{denied:execute(profile,{},'/usr/bin/curl','--noproxy','*','--max-time','2',"http://127.0.0.1:#{port}")};server.close
env={'EASEL_ROOT'=>root,'EASEL_SESSION'=>'offline','EASEL_BOX'=>nil}
[['open','offline'],['do','canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}};p=pile{{"red earth",1}};b=brush("round",4);b:load(p);b:touch(100,80)'],['look'],['note','offline journal'],['status'],['close']].each{|a|r<<execute(profile,env,bin,*a)}
File.write(out,JSON.pretty_generate(r).gsub(root,'<root>').gsub(bin,'<easel>'))
