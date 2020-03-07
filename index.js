var i=0;
var printString;
var moji="CLICK"
function typetext() {
    var test = document.getElementById("typ")
    test.innerText=moji.substring(0,i++);
    if(i<=moji.length) {
        setTimeout("typetext()",100);
    }
    else{
        test.classList.add("test");
    }
}
$('a[href^="#"]').click(function() {
  // スクロールの速度
  var speed = 400; // ミリ秒で記述
  var href = $(this).attr("href");
  var target = $(href == "#" || href == "" ? 'html' : href);
  var position = target.offset().top;
  $('body,html').animate({
    scrollTop: position
  }, speed, 'swing');
  return false;
});