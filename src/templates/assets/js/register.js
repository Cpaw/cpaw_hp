$(function(){
    
    // パスワード確認
    $('button#register').on('click', function(e){
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

        $.ajax({
            type: 'POST',
            url: 'http://localhost:3000/register',
            data: {
                "email": $('input#email').val(),
                "username": $('input#username').val(),
                "password": $('input#password').val(),
                "invite_token": $('input#invite_token').val(),
                "bio": $('textarea#bio').val(),
                "twitter": $('input#twitter').val(),
                "facebook": $('input#facebook').val(),
                "tags": [$('input#tags0').val(), $('input#tags1').val()],
            },
            dataType: 'json',
            success: function(data, textStatus, jqXHR) {
                
                if(data["result"] == true) {
                    window.location.href = '/login';
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
