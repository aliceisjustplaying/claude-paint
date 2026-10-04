require 'open3';require 'json'
dir,out=ARGV
def run(*a);o,e,s=Open3.capture3('magick',*a);raise e unless s.success?;o;end
r={};Dir[dir+'/*.png'].each{|p|r[File.basename(p)]={dimensions:run('identify','-format','%wx%h',p),rgb_sha:Open3.capture3('shasum','-a','256',stdin_data:run(p,'-depth','8','RGB:-'))[0].split.first}}
[['value','gray'],['squint','blur'],['plain','plain-again']].each{|a,b|r[a+'='+b]=run(dir+'/'+a+'.png','-depth','8','RGB:-')==run(dir+'/'+b+'.png','-depth','8','RGB:-')}
r['mirror_exact']=run(dir+'/plain.png','-flop','-depth','8','RGB:-')==run(dir+'/mirror.png','-depth','8','RGB:-')
r['value_mirror_exact']=run(dir+'/value.png','-flop','-depth','8','RGB:-')==run(dir+'/value-mirror.png','-depth','8','RGB:-')
r['blur_mirror_exact']=run(dir+'/squint.png','-flop','-depth','8','RGB:-')==run(dir+'/squint-mirror.png','-depth','8','RGB:-')
r['value_achromatic']=run(dir+'/value.png','-depth','8','RGB:-').bytes.each_slice(3).all?{|a,b,c|a==b && b==c}
r['blur_changed']=r['plain.png'][:rgb_sha]!=r['squint.png'][:rgb_sha]
File.write(out,JSON.pretty_generate(r));puts JSON.pretty_generate(r)
