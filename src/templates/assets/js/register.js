$(function(){

    // パスワード確認
    $('button').on('click', function(e){
        if($('#password').val() !== $('#password_confirm').val()) {
            $('h2').after($('<p></p>').text("パスワードが一致していません。"));
            $('#password').val("");
            $('#password_confirm').val("");
            return false;
        }
    });
});
