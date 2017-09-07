$(function(){
    

    $('button#start').on('click', function(e) {
        
        var cand=[];
        $("[name='user[]']:checked").each(function(){
            cand.push(this.value);
        });
        
        // 配列をシャッフル
        let shuffle = function() {return Math.random()-.5};
        cand.sort(shuffle);

        $('div#content').children().remove();
        $.each(cand, function(i, v) {
            $('div#content').append(
                $('<h3></h3>').text((i+1) + "番目: " + v)
            );
        });
        $("[name='user[]']").prop('checked', false);
    });
});

