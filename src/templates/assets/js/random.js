$(function(){
    $.getJSON('http://localhost:3000/users_json', function(d) {
        usernames = d["usernames"];
        for(var i = usernames.length - 1; i > 0; i--){
            var r = Math.floor(Math.random() * (i + 1));
            var tmp = usernames[i];
            usernames[i] = usernames[r];
            usernames[r] = tmp;
        }
        $.each(usernames, function(i, val) {
            $('div#content').append(
                $('<p style="text-align: center;"></p>').text(i + "番目: " + val)
            );
        });
    });
});

