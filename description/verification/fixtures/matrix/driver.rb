# Run with: ruby marks.rb /absolute/path/to/easel /tmp/owned-root output-directory
require 'open3'; require 'json'; require 'fileutils'; require 'digest'
BIN, ROOT, OUT = ARGV
abort 'binary, disposable root, output required' unless OUT
FileUtils.mkdir_p([ROOT,OUT]); ENV['EASEL_ROOT']=ROOT
$results=[]
def cli(*args)
 t=Process.clock_gettime(Process::CLOCK_MONOTONIC); o,e,s=Open3.capture3(BIN,*args)
 r={args:args,exit:s.exitstatus,out:o.gsub(ROOT,'<root>'),err:e.gsub(ROOT,'<root>'),seconds:(Process.clock_gettime(Process::CLOCK_MONOTONIC)-t).round(4)}
 $results << r; File.write(File.join(OUT,'marks-results.json'),JSON.pretty_generate($results)); r
end
def run(code);cli('do',code);end
def shot(name); p=File.join(OUT,name+'.png');cli('save',p);p;end
def fresh(name,setup='')
 cli('close') if $opened
 cli('open',name.tr('.','_'));$opened=true
 run('canvas{size=100,aspect=5,linen=12,ground={{pile={{"lead white",1}},um=40,apply="knife"}}}; p=pile{{"red earth",1}}; q=pile{{"lead white",1}}; m=rect(300,40,200,100); '+setup)
end
def case_image(name,setup,code)
 fresh(name,setup);r=run(code);shot(name) if r[:exit]==0;r
end
