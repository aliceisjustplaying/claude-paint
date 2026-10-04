require_relative 'driver'
cli('open','closed');cli('close');['b:touch(100,80)','m=rect(1,1,10,10)','h:line({{1,1},{2,2}})','r:blot(100,80)','work(m,{pile=p})','wait(1)'].each{|s|run(s)}
