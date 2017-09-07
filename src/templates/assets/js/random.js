$(function(){
    
    var checks=[];
    $('button#start').on('click', function(e) {
        $("[name='user[]']:checked").each(function(){
            checks.push(this.value);
        });
        alert(checks);
        $.ajax({
            type: 'POST',
            url: 'http://localhost:3000/random',
            data: {
                "usernames": checks
            },
            dataType: "json"
        });
    });
});

