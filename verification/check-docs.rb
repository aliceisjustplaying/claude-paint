# Structural checks for the description, not tests of the painting engine.
require 'pathname'
root=File.expand_path('..',__dir__)
features=Dir[root+'/{foundations,painting,session,watching,delivery}/*.md'].sort
sections=['Summary','The simple case','The interaction, event by event','Modifiers','Cancel and interrupt','Interactions with other systems','Edge cases','Open questions and verification']
interrupts=['Explicit abort','Another action','Environment failure','Target changed externally','Input channel changed']
concerns=['Access','History','Containers','Restricted state','Offline','Collaboration','Notifications','Preferences']
errors=[]
slug=lambda{|s|s.downcase.gsub(/<[^>]*>/,'').gsub(/[^\p{Alnum}_ -]/,'').tr(' ','-')}
features.each do |f|
 text=File.read(f);name=Pathname.new(f).relative_path_from(Pathname.new(root)).to_s
 errors << "#{name}: section order" unless text.scan(/^## (.+)$/).flatten==sections
 block=text.split('## Cancel and interrupt',2)[1].to_s.split(/^## /,2)[0]
 rows=block.lines.grep(/^\|/).drop(2).map{|l|l.split('|')[1].strip}
 errors << "#{name}: interrupt rows" unless rows==interrupts
 block=text.split('## Interactions with other systems',2)[1].to_s.split(/^## /,2)[0]
 errors << "#{name}: concern order" unless block.scan(/^\*\*([^*]+)\.\*\*/).flatten==concerns
 errors << "#{name}: missing diagram" unless text.include?('stateDiagram-v2')
 errors << "#{name}: source commit" unless text.include?('4e525e50897807e9b5f734071dfeb330f1a393d3')
 errors << "#{name}: missing coverage row" unless File.read(root+'/README.md').include?("(#{name})")
end
Dir[root+'/**/*.md'].each do |f|
 text=File.read(f)
 errors << "#{f}: conflict marker" if text.match?(/^(<<<<<<<|=======|>>>>>>>|\|\|\|\|\|\|\|)/)
 text.scan(/\[[^\]]*\]\(([^)]+)\)/).flatten.each do |target|
  next if target.match?(/^(https?:|mailto:)/)
  target=target.delete_prefix('<').delete_suffix('>')
  path,anchor=target.split('#',2);path=path.sub(/:\d+$/,'')
  dest=path.empty? ? f : File.expand_path(path,File.dirname(f))
  unless File.exist?(dest);errors << "#{f}: missing #{target}";next;end
  if anchor && File.extname(dest)=='.md'
   anchors=File.read(dest).scan(/^\#{1,6} (.+)$/).flatten.map{|h|slug.call(h)}
   errors << "#{f}: missing anchor #{target}" unless anchors.include?(anchor)
  end
 end
end
terms=File.read(root+'/glossary.md').scan(/^\*\*([^*]+)\.\*\*/).flatten
errors << 'duplicate glossary terms' unless terms.map(&:downcase).uniq.size==terms.size
puts "#{features.size} feature documents; #{terms.size} glossary terms; #{errors.size} structural/link errors"
puts errors
exit(errors.empty? ? 0 : 1)
