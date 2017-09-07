$(function() {
    /* ログイン処理 */
    $('button#submit').on('click', function(e) {

        
        $.ajax({
            type: 'POST',
            url: 'http://localhost:3000/login',
            data: {
                "username": $('input#username').val(),
                "password": $('input#password').val()
            },
            dataType: 'json',
            success: function(data, textStatus, jqXHR) {
                if(data["result"]) {
                    window.location.href = "/";
                }else{
                    
                    if($('p#errorMsg')[0]) {
                        $('p#errorMsg').remove();
                    }
                    
                    $('h2').after(
                        $('<p></p>').attr('id', "errorMsg").text("メールアドレスまたはパスワードが間違っています")
                    );
                    
                    $('input#username').val("");
                    $('input#password').val("");
                }
            }
        });
    });
    
    $.getJSON('http://localhost:3000/username.json', function(d) {
        username = d["session"];
        if(username != "guest") {
            $('a#login').attr("href", "/user/" + username).text(username).attr('data-title', username);
            $('ul').append($('<li></li>').append(
                $('<a></a>').attr("href", "/logout").attr("data-title", "logout").text("logout"))
                          );
        }
    })
});

