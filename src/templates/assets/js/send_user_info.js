function send_user_info(type, url, add_data, success){
  var data = {
    "email": $('input#email').val(),
    "username": $('input#username').val(),
    "password": $('input#password').val(),
    "bio": $('textarea#bio').val(),
    "twitter": $('input#twitter').val(),
    "facebook": $('input#facebook').val(),
    "tags": [$('input#tags0').val(), $('input#tags1').val()],
    "csrf_token": $('input#csrf_token').attr("value")
  };
  $.extend(data, add_data);

  $.ajax({
    type: type,
    url: url,
    data: data,
    dataType: 'json',
    success: success
  });
}
