# One-off observation harness: unchanged production modules and existing Cargo-built dependencies.
# Arguments: source worktree (with target/debug dependencies), output transcript.
require 'tmpdir';require 'open3'
source,out=ARGV;raise 'source worktree and output required' unless out
Dir.mktmpdir('clock-grain') do |tmp|
 path=tmp+'/main.rs';File.write(path,File.read(File.join(__dir__,'clock-grain.rs')).gsub('@SOURCE@',File.realpath(source)))
 deps=source+'/target/debug/deps';args=['rustc','--edition=2024','--cfg','tube_box','-A','warnings',path,'-L',"dependency=#{deps}"]
 %w[paint mlua image].each{|n|matches=Dir[deps+"/lib#{n}-*.rlib"];raise "ambiguous #{n} dependency" unless matches.length==1;args+=['--extern',n+'='+matches[0]]}
 args+=['-o',tmp+'/probe'];o,e,s=Open3.capture3(*args);raise o+e unless s.success?;o,e,s=Open3.capture3(tmp+'/probe');File.write(out,o+e);raise 'observation failed' unless s.success?
end
