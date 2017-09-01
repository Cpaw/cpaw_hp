$(document).ready(function () {
  	hsize = $(window).width();
	$("#top_active").css("height", hsize / 3.55);
});

$(window).resize(function () {
	hsize = $(window).width();
	$("#top_active").css("height", hsize / 3.55);
});

