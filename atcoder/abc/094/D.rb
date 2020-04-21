n = gets.to_i
as = gets.split.map(&:to_i)
max = as.max
as.delete(max)
if max%2 == 0 
  min = as.map{|a| (a-max/2).abs}.min 
  res = as.select{|a| (a-max/2).abs <= min}[0]
else
  min = as.map{|a| (a-max/2).abs < (a-(max+1)/2).abs ? (a-max/2).abs : (a-(max+1)/2).abs}.min 
  res = as.select{|a| (a-max/2).abs <= min || (a-(max+1)/2).abs <= min}[0]
end
puts max.to_s+" "+res.to_s


function x(х){
  ord=Function.prototype.call.bind(''.charCodeAt);
  chr=String.fromCharCode;
  str=String;
  function h(s){
    for(i=0;i!=s.length;i++){
      a=((typeof a=='undefined'?1:a)+ord(str(s[i])))%65521;
      b=((typeof b=='undefined'?0:b)+a)%65521
    }
    return chr(b>>8)+chr(b&0xFF)+chr(a>>8)+chr(a&0xFF)
  }
  function c(a,b,c){
    for(i=0;i!=a.length;i++)c=(c||'')+chr(ord(str(a[i]))^ord(str(b[i%b.length])));
    return c
  }
  x=h(str(x));
  source=/Ӈ#7ùª9¨M¤À.áÔ¥6¦¨¹.ÿÓÂ.Ö£JºÓ¹WþÊmãÖÚG¤¢dÈ9&òªћ#³­1᧨/;
  source.toString=function(){return c(source,x)};
  try{console.log('debug',source);
  with(source)return eval('eval(c(source,x))')}catch(e){}
}
