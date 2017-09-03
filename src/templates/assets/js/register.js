$(function(){

    // パスワード確認
    $('button').on('click', function(e){

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
    });
});
