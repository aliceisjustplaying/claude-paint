require_relative 'driver'
['plain','custom'].each{|v|case_image('brush-'+v,'custom=brush{kind="round",width=12'+(v=='custom' ? ',stiffness=0.1,bristles=15' : '')+'};b=brush("flat",12);b:load(p);','b:stroke({{200,80},{500,120},{800,80}});print(b)')};cli('close')
