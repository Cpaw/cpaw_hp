

$(function(){
var csvList;
var insert = '';
    $.ajax({
        url: 'assets/csv/member.csv',
        success: function(data) {
            // csvを配列に格納
            csvList = $.csv(",", "", "\n")(data);
            console.log(csvList);
            for (var i = 1; i < csvList.length; i++) {
                console.log(csvList[i][0]);
                insert += '<div class="card"><div class="demo-card-image mdl-card mdl-shadow--2dp"><div class="mdl-card--expand"></div><div class="mdl-card__actions">'
                insert +='<span class="demo-card-image__filename">'+csvList[i][0]+'</span>';
                insert += '</div><div class="mdl-card__skills">';
                insert += csvList[i][2];
                insert += '</div><div class="mdl-hide"><div class="mdl-card__explain">';
                insert += csvList[i][3];
                insert += '</div><div class="mdl-card__links"><p><<br><br></p></div></div></div></div>';
                $(".cards").append(insert);
                insert="";
            }
            
        }
    });
});


    // for (var i = 0; i < csvList.length; i++) {
    //     insert += '<section class="card">';
    //     insert += '<div class="card-content">';
    //     insert += '<img class="card-img" src="img/news.png" alt="">';
    //     insert += '<h1 class="card-title">Webクリエイターボックス</h1>';
    //     insert += '    <p class="card-text">ユイアdlsfjぁしdj</p>';
    //     insert += '</div>';
    //     insert += '<div class="card-link">';
    //     insert +='<a href="news1.html" class="widelink">続きを読む</a>';
    //     insert += '  </div></section>';
    //     $("#box2").append(insert);
    //     insert="";
    // }


