eval(File.read(File.join(__dir__,'root.rb')).split("n='views';")[0])
source=File.read(File.join(__dir__,"channel-input.lua")).strip;r=[]
%w[inline file stdin].each do |n|
 ok(n,'open',n);f=ROOT+'/input.lua';File.write(f,source);a=case n;when 'file';cli(n,'do','-f',f);when 'stdin';cli(n,'do','-',stdin:source);else cli(n,'do',source);end
 r<<{channel:n,source:source,reply:a,status:cli(n,'status'),look:look(n,n,'--mode','value,mirror','--grid','50','--size','80'),log:cli(n,'log')};ok(n,'close')
end
rec('channels',{runs:r,tool_png_sha:'8b31006d8dda548e21cf97af4aaf50844d899daff18b27ca38ddf433a610e357'})
