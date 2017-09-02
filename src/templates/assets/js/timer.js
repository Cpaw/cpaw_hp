$(function(){

    var setSecond = 300; //タイマーの秒数
    var setPause = setSecond; //ストップ時の秒数を保存する変数　初期値はsetSecondと同じ数値
    var time = setSecond;   //残り秒数を保存する変数　初期値はsetSecondと同じ数値
    var timerID;    //setInterval用の変数



    //関数の設定-----------------------------------

    //残り秒数を表示させる関数
    function textDisplay(){
        var minute = Math.floor(time / 60);
        var second = time % 60;
        $("#countDown").text(minute + "分 " + second + "秒");
           
    };

    //カウントを1減らす関数（setIntervalで毎秒実行される関数）
    function countDown(){
        time--;  //残り秒数を1減らす
        setPause = time;  //setPauseに残り秒数を代入（ストップ時に使用するため）
        textDisplay();    //1減った残り秒数を表示
        
    }

    //タイマー（setInterval）の停止用関数
    function countStop(){
        clearInterval(timerID); //（setInterval）をクリアー
        
    }

    //タイマースタートの関数
    function timerStart(){
        countStop();   //カウントダウンの重複を防ぐために今動いているタイマーをクリアーする ※1
        timerID = setInterval(function(){
            if(time <= 0) {
                //残り秒数が0以下になったらタイマー（setInterval）をクリアー
                clearInterval(timerID);
                         
            } else {
                //残り秒数が1以上あれば1減らす
                countDown();
                         
            }

            
        }, 1000);   //1000ミリ秒（1秒）毎にsetInterval内の処理を繰り返す
        
    };



    //実行処理-----------------------------------

    textDisplay();

    //スタートボタンクリックでタイマースタート
    $("#startBtn").click(function(){
        time = setPause; //setPauseに入っている秒数から開始
        textDisplay();
        timerStart();
           
    });

    //ストップボタンクリックでタイマー停止
    $("#stopBtn").click(function(){
        countStop();
           
    });

    //リセットボタンクリックでタイマー初期化
    $("#resetBtn").click(function(){
        countStop();
        time = setPause = setSecond; //setSecondに入っている秒数を代入し直す
        textDisplay();
           
    });

    //保存ボタンクリックで秒数変更フォームの入力チェック
    $("#changeSecond").click(function(){

        //入力欄（#setSecond input）に記入された内容をseveSecondに代入
        var seveSecond = $("#setSecond").val() * 60;
        

        //正規表現を使用して半角数字か判別を行う
        if(seveSecond.match( /[^0-9]+/ )){
            //半角数字以外のものが含まれていた場合、エラーテキストを表示
            $("#error").text("※半角数字で入力してください")

            //seveSecondが空でないか判別を行う
                   
        } else if(seveSecond=='') {
            //何も入力されてない場合、エラーテキストを表示
            $("#error").text("※値を入力してください")

            //入力が問題ない場合
                   
        } else {
            //エラーテキストを空に
            $("#error").text("")
            //入力された値（seveSecond）をタイマーの秒数（setSecond）に設定
            setSecond = seveSecond;
            //以下リセットボタン押下時と同じ処理
            countStop();
            time = setPause = setSecond;
            textDisplay();
                   
        }
           
    });


});
