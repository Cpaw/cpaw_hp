$(function(){

  // パスワード確認
  $('button#apply').on('click', function(e){
    // パスワードが一致していません文章が無限に増えないようにする
    if($('p#errorMsg')[0]) {
      $('p#errorMsg').remove();
    }

    if($('#email').val() === ""){
      $('h2').after(
        $('<p></p>').attr('id', "errorMsg").text("メールアドレスが空です。")
      );
      return false;
    }

    if($('#username').val() === ""){
      $('h2').after(
        $('<p></p>').attr('id', "errorMsg").text("ユーザー名が空です。")
      );
      return false;
    }

    if($('#password').val() !== ""
      && $('#password').val() !== $('#password_confirm').val()) {

      $('h2').after(
        $('<p></p>').attr('id', "errorMsg").text("パスワードが一致していません。")
      );
      $('#password').val("");
      $('#password_confirm').val("");
      return false;
    }

    var username = location.pathname.split('/').pop();

    $.ajax({
      type: 'PATCH',
      url: 'http://localhost:3000/user/' + username,
      data: {
        "email": $('input#email').val(),
        "username": $('input#username').val(),
        "password": $('input#password').val(),
        "bio": $('textarea#bio').val()
      },
      dataType: 'json',
      success: function(data, textStatus, jqXHR) {
        if(data["result"] == true) {
          window.location.href = '/user/'+data["username"];
        }

        if($('p#errorMsg')[0]) {
          $('p#errorMsg').remove();
        }
        $('h2').after(
          $('<p></p>').attr('id', "errorMsg").text(data["result"])
        );
      }
    });
  });
});
