$(function(){
    
    // パスワード確認
    $('button#submit').on('click', function(e){
        // パスワードが一致していません文章が無限に増えないようにする
        if($('p#errorMsg')[0]) {
            $('p#errorMsg').remove();
        }
        if($('#password').val() !== $('#password_confirm').val()) {
            $('h2').after(
                $('<p></p>').attr('id', "errorMsg").text("パスワードが一致していません。")
            );
            $('#password').val("");
            $('#password_confirm').val("");
            return false;
        }

        var success = function(data, textStatus, jqXHR) {
          if(data["result"] == true) {
            window.location.href = '/login';
          }

          if($('p#errorMsg')[0]) {
            $('p#errorMsg').remove();
          }
          $('h2').after(
            $('<p></p>').attr('id', "errorMsg").text(data["result"])
          );
        };

        send_user_info('POST', '/register', {'invite_token': $('input#invite_token').val()}, success);
    });
});
