require 'open3';require 'json';require 'digest'
# Pixel receipts use decoded RGB, not compressed-file equality.
def pix(p);o,e,s=Open3.capture3('magick',p,'-depth','8','RGB:-');raise e unless s.success?;o;end
def compare(a,b)
 x=pix(a);y=pix(b);n=0;bounds=[2400,480,-1,-1];(x.size/3).times{|i|next if x.byteslice(i*3,3)==y.byteslice(i*3,3);n+=1;px=i%2400;py=i/2400;bounds=[bounds[0]<px ? bounds[0]:px,bounds[1]<py ? bounds[1]:py,bounds[2]>px ? bounds[2]:px,bounds[3]>py ? bounds[3]:py]};{changed:n,bounds_px:bounds}
end
base=ARGV[1] or abort 'output JSON and image directory required';out={}
[['stroke-default','stroke-explicit'],['touch-default','touch-explicit'],['reload-reload','reload-wipe85'],['retained-second','retained-wipe'],['line-default','line-explicit'],['line-default','line-smooth'],['sketch-default','sketch-explicit'],['hatch-default','hatch-explicit'],['sketch-0','sketch-1'],['sketch-12','sketch-99'],['erase--1','erase-0'],['erase-1','erase-2'],['erase-mask-4','erase-mask-20'],['rag-pressure--1','rag-pressure-0'],['rag-pressure-1','rag-pressure-2'],['rag-blot','rag-one'],['rag-one','rag-short']].each{|a,b|out["#{a}/#{b}"]=compare("#{base}/#{a}.png","#{base}/#{b}.png")}
Dir[base+'/clip-*.png'].each{|p|out[File.basename(p)]=compare(base+'/blank.png',p)}
File.write(ARGV[0],JSON.pretty_generate(out));puts JSON.pretty_generate(out)
