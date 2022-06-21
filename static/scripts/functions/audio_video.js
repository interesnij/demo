
function msToTime(duration) {
  var milliseconds = parseInt((duration % 1000) / 100),
    seconds = Math.floor((duration / 1000) % 60),
    minutes = Math.floor((duration / (1000 * 60)) % 60);
  minutes = (minutes < 10) ? "0" + minutes : minutes;
  seconds = (seconds < 10) ? "0" + seconds : seconds;
  return minutes + ":" + seconds;
};

function get_video_dop(){
  styles = document.querySelectorAll(".my_color_settings");
  style = styles[styles.length- 1];
  settings = [];
  if (style.href.indexOf("a.css") != -1){
    settings += ["images/video_white",'#eeeeee','#FFFFFF']
  } else {
    settings += ["images/video_dark",'#000000','#000000']
  }
  return settings.split(',')
};
function get_audio_dop(){
  styles = document.querySelectorAll(".my_color_settings");
  style = styles[styles.length- 1];
  settings = [];
  if (style.href.indexOf("a.css") != -1){
    settings += ["images/audio_white",'#eeeeee','#FFFFFF']
  } else {
    settings += ["images/audio_dark",'#000000','#000000']
  }
  return settings.split(',')
};

function play_video_list(url, counter, video_pk){
  loader = document.getElementById("video_loader");
  open_video_fullscreen(url);

  video_player_id = document.body.getAttribute('data-video');
  document.body.setAttribute('data-video', document.body.getAttribute('data-video') + "a");
  setTimeout(function() {
    load_video_playlist(video_player_id + "a");
    video_player.addListener(FWDUVPlayer.READY, onReady);
    function onReady(){
    console.log("video player ready");
    setTimeout(function() {video_player.playVideo(counter)}, 1000);

    info_video = document.body.querySelector("#info_video");
    if (info_video.innerHTML == "" || info_video.getAttribute("data-pk") != video_pk){
      info_video.setAttribute("data-pk", video_pk);
      console.log("Воспроизводится ролик № : " + video_pk)
    }
    }
  }, 500);
  video = document.createElement("div");
  video.classList.add("video_init");
  document.body.querySelector("#video_loader").append(video)
};

function load_video_playlist(video_saver_id) {
  video_saver = document.body.querySelector("#video_id_saver");
  styles = document.querySelectorAll(".my_color_settings");
  style = styles[styles.length- 1];

video_player = new FWDUVPlayer({
    instanceName:video_saver_id,
    parentId: "video_player",
    playlistsId:"video_playlists",
    skinPath: get_video_dop()[0],
    mainFolderPath:"/static",
    displayType:"responsive",                 // тип дисплея (выбран отзывчивый к размерам экрана)
    useVectorIcons:"no",                      // использование векторной графики
    fillEntireVideoScreen:"no",               // заполнение всего экрана видео-роликом
    fillEntireposterScreen:"yes",             // заполнение всего экрана постером
    goFullScreenOnButtonPlay:"no",            // показывать кнопку включения полноэкранного режима
    playsinline:"yes",                        // играет в ряд
    initializeOnlyWhenVisible:"no",           // инициализировать плеер только тогда, когда он виден
    youtubeAPIKey:'AIzaSyCgbixU3aIWCkiZ76h_E-XpEGig5mFhnVY', // ключ разработчика для ютуба
    useHEXColorsForSkin:"no",                 // использование hex кодировки для скина
    normalHEXButtonsColor:"#FF0000",          // цвет кнопки
    selectedHEXButtonsColor:"#000000",        // цвет нажатой кнопки
    useResumeOnPlay:"no",                     // использование резюме при проигрывании
    useDeepLinking:"no",                      // использование глубоких ссылок для ограничения перехвата ссылки на видео
    showPreloader:"yes",                      // gjrfpsdfnm ghtkjflth ghb pfuheprt gktthf
    preloaderBackgroundColor:"#000000",       // цвет фона прелоадера
    preloaderFillColor:"#FFFFFF",             // цвет прелоадера
    addKeyboardSupport:"no",                 // использовать поддержку клавиатуры
    autoScale:"yes",                          // автоматическое масштабирование
    stopVideoWhenPlayComplete:"no",           // остановить плеер после проигрывания последнего ролика
    playAfterVideoStop:"yes",                 // воспроизведение после остановки видео
    autoPlay:"no",                            // автоматический старт проигрывания
    loop:"no",                                // повтор видео сразу
    shuffle:"no",                             // перемешивание видео сразу
    showErrorInfo:"no",                       // показывать информацию об ошибках
    maxWidth:1170,                            // максимальная ширина
    maxHeight:659,                            // максимальная высота
    volume:.8,                                // начальная громкость плеера (1 - 100%)
    backgroundColor:get_video_dop()[1],                // цвет фона
    videoBackgroundColor:"#000000",           // цвет фона видео-секции
    posterBackgroundColor:"#000000",          // цвет фона постера

    //logo settings
    showLogo:"no",                            // показывать логотип над секцией видео справа

    //playlists/categories settings
    showPlaylistsSearchInput:"no",            // показывать поле поиска плейлиста
    usePlaylistsSelectBox:"no",              // использовать выбор плейлистов в окне сверху
    showPlaylistsByDefault:"no",              // показать плейлист по умолчанию
    thumbnailSelectedType:"opacity",          // анимация выбранного плейлиста в окне сверху
    startAtPlaylist:0,                        // проигрывать плейлист номер ...
    buttonsMargins:15,                        // расстояние между кнопками
    thumbnailMaxWidth:350,                    // максимальная ширина миниатюры
    thumbnailMaxHeight:350,                   // максимальная высота миниатюры
    horizontalSpaceBetweenThumbnails:40,      // расстояние между миниатюрами по горизонтали
    verticalSpaceBetweenThumbnails:40,        // расстояние между миниатюрами по вертикали
    inputBackgroundColor:"#333333",           // цвет фона поля ввода
    inputColor:"#000000",                     // цвет текста поля ввода

    //playlist settings
    showPlaylistButtonAndPlaylist:"yes",      // показывать кнопку выбора плейлистов и сами плейлисты сверху
    playlistPosition:"right",                 // расположение плейлиста
    showPlaylistByDefault:"yes",              // показать плейлист по умолчанию
    showPlaylistName:"no",                   // показывать название плейлиста
    showSearchInput:"no",                    // показывать поле поиска
    showLoopButton:"yes",                     // показывать кнопку повтора
    showShuffleButton:"yes",                  // показывать кнопку перемешивания
    showPlaylistOnFullScreen:"no",            // показывать плейлист в режиме полного экрана
    showNextAndPrevButtons:"yes",             // показывать кнопки пред/след видео
    showThumbnail:"yes",                      // показывать миниатюры
    addMouseWheelSupport:"yes",               // поддержка управления мыши
    startAtRandomVideo:"no",                  // начинать воспроиведение со случайного видео ролика
    stopAfterLastVideoHasPlayed:"no",         // останавливать воспроизведение после последнего ролика
    addScrollOnMouseMove:"no",                // перемотка движениями мыши
    randomizePlaylist:'no',                   // случайные плейлисты
    folderVideoLabel:"VIDEO ",                // название папки видео
    playlistRightWidth:320,                   // ширина плейлиста справа
    playlistBottomHeight:380,                 // высота плейлиста снизу
    startAtVideo:0,                           // начинать с ролика номер ...
    maxPlaylistItems:50,                      // максимальное количество роликов в плейлисте
    thumbnailWidth:71,                        // ширина миниатюры
    thumbnailHeight:71,                       // высота миниатюры
    spaceBetweenControllerAndPlaylist:1,      // расстояние между контроллером и плейлистом
    spaceBetweenThumbnails:1,                 // расстояние между миниатюрами
    scrollbarOffestWidth:8,                   // отступ ширины скроллбара
    scollbarSpeedSensitivity:.5,              // скорость отклика скроллбара
    playlistBackgroundColor:get_video_dop()[1],         // цвет фона плейлиста
    playlistNameColor:get_video_dop()[1],              // цвет названия плейлиста
    thumbnailNormalBackgroundColor:get_video_dop()[2], // цвет фона миниатюры
    thumbnailHoverBackgroundColor:get_video_dop()[1],  // цвет фона активной миниатюры
    thumbnailDisabledBackgroundColor:get_video_dop()[1], // цвет фона disabled миниатюры
    youtubeAndFolderVideoTitleColor:get_video_dop()[1],// цвет плейлиста роликов с папок и ютуба
    youtubeOwnerColor:"#919191",              // цвет названия ролика я ютуба
    youtubeDescriptionColor:"#919191",        // цвет описания ролика я ютуба
    mainSelectorBackgroundSelectedColor:get_video_dop()[2], // цвет фона плейлиста при наведении
    mainSelectorTextNormalColor:get_video_dop()[1],    // цвет текста плейлиста
    mainSelectorTextSelectedColor:get_video_dop()[2],
    mainButtonBackgroundNormalColor:get_video_dop()[2],// цвет фона кнопок
    mainButtonBackgroundSelectedColor:get_video_dop()[2],// цвет фона нажатой кнопки
    mainButtonTextNormalColor:get_video_dop()[2],      // цвет текста кнопок
    mainButtonTextSelectedColor:get_video_dop()[2],    // цвет текста нажатой кнопки

    //controller settings
    showController:"yes",                     // показывать контроллер
    showControllerWhenVideoIsStopped:"yes",   // показывать контроллер при остановке проигрывания
    showNextAndPrevButtonsInController:"no",  // показывать кнопки пред / след на контроллере
    showRewindButton:"yes",                   // показать кнопку перемотки назад
    showPlaybackRateButton:"yes",             // показать кнопку выбора скорости воспроизведения
    showVolumeButton:"yes",                   // показать кнопку громкости
    showTime:"yes",                           // показать время воспроизведения
    showQualityButton:"yes",                  // показать время выбора качества видео
    showInfoButton:"no",                     // показывать кнопку информации ролика
    showShareButton:"no",                     // показывать кнопку расшаривания ролика
    showEmbedButton:"no",                    // показывать кнопку получения ссылки ролика и фрейма для вставки на другие сайты
    showChromecastButton:"no",                // показывать кнопку подкастов
    showFullScreenButton:"yes",               // показывать кнопку полноэкранного режима
    disableVideoScrubber:"no",                // выключить ползунок переключения времени видео
    showScrubberWhenControllerIsHidden:"yes", // показывать ползунок времени воспроизведенного ролика при скрытом контроллере
    showDefaultControllerForVimeo:"no",       // показывать контроллер vimeo
    repeatBackground:"yes",                   // повтор бекгроунда
    controllerHeight:42,                      // высота контроллера
    controllerHideDelay:3,                    // время, через которое скроется контроллер
    startSpaceBetweenButtons:7,               // начальное расстояние между кнопками
    spaceBetweenButtons:8,                    // расстояние между кнопками
    scrubbersOffsetWidth:2,                   // ширина отступа скруббера
    mainScrubberOffestTop:14,                 // отступ скруббера всерху
    timeOffsetLeftWidth:5,                    // ширина отступа времени воспроизведения слева
    timeOffsetRightWidth:3,                   // ширина отступа времени воспроизведения справа
    timeOffsetTop:0,                          // отступ времени воспроизведения сверху
    volumeScrubberHeight:80,                  // высота скруббера громкости
    volumeScrubberOfsetHeight:12,             // отступскруббера громкости по высоте
    timeColor:"#919191",                      // цвет времени воспроизведения
    youtubeQualityButtonNormalColor:"#919191",// кнопка выбора качества плейлитса ютуба
    youtubeQualityButtonSelectedColor:"#000000",// нажатая кнопка выбора качества плейлитса ютуба

    //advertisement on pause window
    aopwTitle:"Advertisement",                // название рекламной вставки
    aopwWidth:400,                            // ширина вставки
    aopwHeight:240,                           // высота вставки
    aopwBorderSize:6,                         // размер рамки вставки
    aopwTitleColor:"#000000",                 // цветназвания вставки

    //subtitle
    subtitlesOffLabel:"Субтитры откл.",       // надпись, когда субтитры отключены

    //popup add windows
    showPopupAdsCloseButton:"no",            // показать кнопку закрытия окна подставки

    //окно размещения и информации
    embedAndInfoWindowCloseButtonMargins:15,  // отступ кнопки закрытия
    borderColor:"#CDCDCD",                    // цвет рамки
    mainLabelsColor:"#000000",                // цвет названия
    secondaryLabelsColor:"#444444",           // вторичный цвет названия
    shareAndEmbedTextColor:"#777777",         // цвет тектса овна расшаривания и вставки
    inputBackgroundColor:"#c0c0c0",           // цвет фона поля ввода
    inputColor:"#333333",                     // цвет фона текста ввода

    //audio visualizer
    audioVisualizerLinesColor:"#ff9f00",      // цвет линий аудио визуализатора
    audioVisualizerCircleColor:"#FFFFFF",     // цвет кругов аудио визуализатора

    //lightbox settings
    lightBoxBackgroundOpacity:.6,             // прозрачность
    lightBoxBackgroundColor:"#000000",        // цвет фона

    //sticky on scroll
    stickyOnScroll:"no",                      // липкое листание
    stickyOnScrollShowOpener:"yes",           // показывать эффект
    stickyOnScrollWidth:"700",                // ширина
    stickyOnScrollHeight:"394",               // высота

    //настройки липкого дисплея
    showOpener:"yes",                         // показывать вставки
    showOpenerPlayPauseButton:"yes",          // показывать кнопку плей при паузе
    verticalPosition:"bottom",                // позиция по вертикали
    horizontalPosition:"center",              // позиция по горизонтали
    showPlayerByDefault:"yes",                // показывать плеер по умолчанию
    animatePlayer:"yes",                      // анимировать плеер
    openerAlignment:"right",                  // выравнивание вставки
    mainBackgroundImagePath:"main-background.png", // путь до изображения фона
    openerEqulizerOffsetTop:-1,               // отступ эквалайзера сверху
    openerEqulizerOffsetLeft:3,               // отступ эквалайзера слева
    offsetX:0,                                // отступ по оси X
    offsetY:0,																// отступ по оси Y

    //скорость воспроизведения
    defaultPlaybackRate:1,                   //0.25, 0.5, 1, 1.25, 1.2, 2
    //cuepoints
    executeCuepointsOnlyOnce:"no",           // выполнение ключевых точек только один раз
    //annotations
    showAnnotationsPositionTool:"no",        // показывать координаты аннотаций на экране

    //ads
    openNewPageAtTheEndOfTheAds:"no",        // открыть новую страницу в конце объявления
    adsButtonsPosition:"left",               // позиция окна рекламы
    skipToVideoText:"Закрыть через: ",       // текст окна рекламы
    skipToVideoButtonText:"Закрыть",         // текст кнопки закрытия рекламного окна
    adsTextNormalColor:"#888888",            // цвет рекламного текста
    adsTextSelectedColor:"#000000",          // цвет выбранного текста
    adsBorderNormalColor:"#AAAAAA",          // цвет рамки рекламного окна
    adsBorderSelectedColor:"#000000",        // цвет выбраной рамки рекламного окна

    //a to b loop
    useAToB:"no",                            // использование повтора от...до
    atbTimeBackgroundColor:"transparent",    // время фона от...до
    atbTimeTextColorNormal:"#888888",        // время текста от...до
    atbTimeTextColorSelected:"#FFFFFF",
    atbButtonTextNormalColor:"#888888",
    atbButtonTextSelectedColor:"#FFFFFF",
    atbButtonBackgroundNormalColor:"#FFFFFF",
    atbButtonBackgroundSelectedColor:"#000000",

    //thumbnails preview
    thumbnailsPreviewWidth:196,              // ширина предпросмотра миниатюры
    thumbnailsPreviewHeight:110,             // высота предпросмотра миниатюры
    thumbnailsPreviewBackgroundColor:get_video_dop()[1],// цвет фона  миниатюры
    thumbnailsPreviewBorderColor:"#666",     // цвет названия миниатюры
    thumbnailsPreviewLabelBackgroundColor:"#666", // цвет фона названия минатюры
    thumbnailsPreviewLabelFontColor:"#FFF",
    // context menu
    showContextmenu:'no',
    showScriptDeveloper:"no",
    contextMenuBackgroundColor:"#ebebeb",
    contextMenuBorderColor:"#ebebeb",
    contextMenuSpacerColor:"#CCC",
    contextMenuItemNormalColor:"#888888",
    contextMenuItemSelectedColor:"#000",
    contextMenuItemDisabledColor:"#BBB"
});
};

function get_resize_screen(){
  video_player.maxWidth = 360;
  video_player.maxHeight = 270;
  video_player.showPlaylist();
};
function get_normal_screen(){
  video_player.maxWidth = 1170;
  video_player.maxHeight = 659;
  video_player.hidePlaylist();
};

music_player = new FWDMSP({
    //main settings
    instanceName:"player1",
    playlistsId:"audio_playlists",
    mainFolderPath:"/static/",
    skinPath:get_audio_dop()[0],
    showSoundCloudUserNameInTitle:"no",   // показывать имя пользователя soundcloud
    showMainBackground:"no",  						// показать общий фон
    verticalPosition:"bottom",            // расположение плеера
    rightClickContextMenu:"developer",
    useDeepLinking:"no",									// использовать глубокие ссылки - защита от перехвата. Не будет работать с souncloud
    rightClickContextMenu:"no",           // показ контекстног меню по щелчку правой кнопкой мыши
    addKeyboardSupport:"yes",             // добавить поддержку клавиатуры
    animate:"yes",												// фнимация
    autoPlay:"no",												// автостарт плеера
    loop:"no",														// повтор песни
    shuffle:"no",													// перемешивание треков
    maxWidth:850,                         // максимальная ширина
    volume:.8,														// громкость по умолчанию 80%

    // controller settings
    showControllerByDefault:"no",        // показать контроллер по умолчанию
    showThumbnail:"yes",                  // показывать миниатюры
    showNextAndPrevButtons:"yes",					// показывать кнопки переключения треков
    showSoundAnimation:"yes",							// показывать анимацию музыкального воспроизведения
    showLoopButton:"yes",                 // показывать кнопку повтора треков
    showShuffleButton:"yes",              // показывать кнопку перемешивания треков
    expandBackground:"no",
    showBuyButton:"yes",
    showPlaylistItemBuyButton:"no",
    titleColor:get_audio_dop()[3],        // цвет названия
    timeColor:"#919191",                  // цвет времени

    // настройки выравнивания и размера контроллера (подробно описаны в документации!)
    controllerHeight:76,                 // высота контроллера
    startSpaceBetweenButtons:9,          // начальное пространство между кнопками
    spaceBetweenButtons:8,               // пространство между кнопками
    separatorOffsetOutSpace:5,           // смещение разделителя вне пространства
    separatorOffsetInSpace:9,            // смещение разделителя в пространстве
    lastButtonsOffsetTop:14,             // смещение последних кнопок сверху
    allButtonsOffsetTopAndBottom:14,     // смещение всех кнопок вверх и вниз
    titleBarOffsetTop:13,                // смещение сверху секции названия
    mainScrubberOffsetTop:47,            // смещение сверху скруббера
    spaceBetweenMainScrubberAndTime:10,  // пространство между скруббером и секцией времени
    startTimeSpace:10,                   // пространство относительно начала в секуии времени
    scrubbersOffsetWidth:2,              // ширина смещения скруббера
    scrubbersOffestTotalWidth:0,         // общая ширина смещения скруббера
    volumeButtonAndScrubberOffsetTop:47, // всещение сверху скруббера и кнопки громкости
    spaceBetweenVolumeButtonAndScrubber:6,// пространство между скруббером и кнопкой громмкости
    volumeScrubberOffestWidth:4,         // смещение слева скруббера громкости
    scrubberOffsetBottom:10,             // сммещение скруббера снизу
    equlizerOffsetLeft:1,                // смещение эквалайзера влево

    //playlists window settings
    showPlaylistsSearchInput:"no",      // показывать поле поиска в плейлисте
    usePlaylistsSelectBox:"no",         // показывать поле плейлистов сверху выбранного плейлиста
    showPlaylistsSelectBoxNumbers:"no", // пронумеровать плейлисты в поле выбора плейлиста
    showPlaylistsButtonAndPlaylists:"yes", // показывать кнопку, вызывающую окно плейлистов сверху и сами плейлисты
    showPlaylistsByDefault:"no",         // показывать плейлист по умолчанию
    thumbnailSelectedType:"opacity",     // тип выбора миниатюры (к примеру прозрачность)
    startAtPlaylist:0,                   // воспроизводить с плейлиста номер...
    startAtTrack:0,                      // воспроизводить с трека номер...
    startAtRandomTrack:"no",             // воспроизводить со случайного трека...
    buttonsMargins:0,                    // отступы кнопок
    thumbnailMaxWidth:330,               // макс. ширина миниатюр
    thumbnailMaxHeight:330,              // макс. высота миниатюр
    horizontalSpaceBetweenThumbnails:40, // пространство между миниатюрами по горизонтали
    verticalSpaceBetweenThumbnails:40,   // пространство между миниатюрами по вертикали
    mainSelectorBackgroundSelectedColor:get_audio_dop()[3], // цвет фона выбранного
    mainSelectorTextNormalColor:"#737373",  // цвет текста селектора
    mainSelectorTextSelectedColor:get_audio_dop()[3], // цвет текста селектора выбранного
    mainButtonTextNormalColor:"#7C7C7C", // цвет текста кнопок
    mainButtonTextSelectedColor:get_audio_dop()[3], // цвет текста кнопок выбранных

    //playlist settings
    playTrackAfterPlaylistLoad:"no",     // воспроизведение трека после загрузки плейлиста
    //showPlayListButtonAndPlaylist:"yes",
    showPlayListOnAndroid:"yes",         // показывать плейлисты на android
    showPlayListByDefault:"no",          // показывть плейлист по умолчанию
    showPlaylistItemPlayButton:"yes",    // показать кнопку воспроизведения элемента плейлиста
    addScrollBarMouseWheelSupport:"yes",  // прокручивать колесиком мыши
    showTracksNumbers:"no",							// показывать номер трека
    playlistBackgroundColor:get_audio_dop()[3],    // цвет фона плейлиста
    trackTitleNormalColor:"#737373",      // цвет заголовка трека
    trackTitleSelectedColor:get_audio_dop()[3],    // цвет заголовка выбранного трека
    trackDurationColor:"#7C7C7C",         // цвет времени трека
    maxPlaylistItems:1000,                  // Макс. количество плейлистов
    nrOfVisiblePlaylistItems:12,          // число видимых элементов списка воспроизведения
    trackTitleOffsetLeft:0,               // смещение слева заголовка трека
    playPauseButtonOffsetLeftAndRight:11, // смещение слева и справа кнопки play
    durationOffsetRight:9,							  // смещение справа продолжительности трека
    scrollbarOffestWidth:7,               // шмрмна смещения полосы прокрутки

    //playback rate / speed
    showPlaybackRateButton:"yes",         // показать кнопку скорости воспроизведения
    defaultPlaybackRate:1, //min - 0.5 / max - 3 // скорость воспроизведения по умолчанию (от 0,5 до 3)
    playbackRateWindowTextColor:"#888888",// цвет текста на окне выбора скорости

    showSearchBar:"no",                  // показывать секцию поиска треков
    showSortButtons:"yes",                // показывать секцию сортировки треков
    searchInputColor:"#999999",						// цвет секции поиска
    searchBarHeight:38,									  // высота секции поиска
    inputSearchTextOffsetTop:1,           // смещение текста ввода поиска сверху
    inputSearchOffsetLeft:0,              // смещение текста ввода поиска слева

    openerAlignment:"right",              // открывание
    showOpener:"yes",                     // показывать эффект
    showOpenerPlayPauseButton:"yes",      // показывать кнопку плей / пауза
    openerEqulizerOffsetLeft:3,           // сдвигание эквалайзера слева
    openerEqulizerOffsetTop:-1,           // сдвигание эквалайзера сверху

    atbTimeBackgroundColor:"transparent", // цвет фона "от / до"
    atbTimeTextColorNormal:"#888888",     // цвет текста "от / до"
    atbTimeTextColorSelected:get_audio_dop()[3],   // цвет выбранного текста "от / до"
    atbButtonTextNormalColor:"#888888",   // цвет кнопки "от / до"
    atbButtonTextSelectedColor:get_audio_dop()[3], // цвет выбранной кнопки "от / до"
    atbButtonBackgroundNormalColor:get_audio_dop()[3], // цвет фона кнопки "от / до"
    atbButtonBackgroundSelectedColor:get_audio_dop()[3], // цвет фона выбранной кнопки "от / до"
  });

FWDMSPUtils.onReady(function() {
  music_player.addListener(FWDMSP.READY, music_onReady);
  music_player.addListener(FWDMSP.PLAY, music_onPlay);
  music_player.addListener(FWDMSP.PAUSE, music_onPause);
});

///////////////////////////
$playlist = document.body.querySelector("#saved_playlist");

function show_active_track(block, list_pk, track_pk) {
  for (i=0; i < playlists.length; i++) {
    if (playlists[i].getAttribute("data-pk") == list_pk) {
      if (playlists[i].classList.contains("is_paginate")) {
        console.log("Обнаружен проигрываемый список плейлиста!");
      }
      else {
        playlists[i].classList.add("play_list", "gradient-border");
      }
    }
  }

  try {
    for (i=0; i < items.length; i++) {
      if (items[i].getAttribute("track-pk") == track_pk) {
        items[i].classList.add("play_track", "gradient-border");
      }
    }
  } catch { null };
};

function remove_play_items(block) {
  // уберем все подсветки плейлистов и треков
  items = block.querySelectorAll('.track');
  for (i=0; i < items.length; i++) {
    items[i].classList.remove("play_track", "gradient-border", "pause");
  }

  playlists = block.querySelectorAll('.playlist');
  for (i=0; i < playlists.length; i++) {
    playlists[i].classList.remove("play_list", "gradient-border", "pause");
  }
  // ---- //
};

function show_play_items(block, track_id, counter) {
  if (counter == undefined) {
    return;
  }

  type = $playlist.getAttribute("data-type");
  pk = type.slice(3);

  // у пользователя выбран плейлист.
  if (type.indexOf('lis') !== -1) {
    if (block.querySelector('[playlist-pk=' + '"' + pk + '"' + ']')) {
      list_id = pk;
    }
  }
  // выбран плейлист записи
  else if (type.indexOf('pos') !== -1) {
    posts = block.querySelectorAll('.post');
    for (i=0; i < posts.length; i++) {
      if (posts[i].getAttribute("track-pk") == track_id) {
        attach_block = posts[i].querySelector('.attach_container');
        list_id = attach_block.querySelector('[playlist-pk=' + '"' + pk + '"' + ']').getAttribute("playlist-pk");
        break;
      }
    }
  }
  console.log("play list" + list_id);
  // отражаем проигрываемый трек и плейлист
  remove_play_items(block);
  show_active_track(block, list_id, track_id);
}

function get_music_player_support(block) {
  if (music_player.getCurrentTime() && music_player.getCurrentTime() != "00:00") {
    show_play_items(block);
  }
}

function music_onReady(){console.log("Аудио плеер готов");}

function music_onPause() {
  try {
    document.title = "Музыка приостановлена";
    if (document.querySelector(".user_status")) {
      document.querySelector(".user_status").innerHTML = "Музыка приостановлена";
    }
  } catch { null }
};

function music_onPlay() {
  var track_id = music_player.buy();
  remove_play_items(document.body);
  show_play_items(document.body, music_player.getTrackId(), );
  try { video_player.pause() } catch { null };
  $playlist.setAttribute("track-pk", track_id);

  title = music_player.getTrackTitle();
  if (title != undefined) {
    title_replace = title.replace(/<\/?[^>]+(>|$)/g, "");
    document.title = "¶ " + title_replace;
    if (document.querySelector(".user_status")) {
      document.querySelector(".user_status").innerHTML = "¶ " + title_replace;
    }
  }
};

function dragElement(elmnt){var pos1=0,pos2=0,pos3=0,pos4=0;document.querySelector("#draggable-header").onmousedown=dragMouseDown;document.querySelector("#draggable-resize").onmousedown=resizeMouseDown;function dragMouseDown(e){e=e||window.event;e.preventDefault();pos3=e.clientX;pos4=e.clientY;document.onmouseup=closeDragElement;document.onmousemove=elementDrag}function resizeMouseDown(e){e=e||window.event;e.preventDefault();pos3=0;pos4=0;document.onmouseup=closeDragElement;document.onmousemove=elementResize}function elementResize(e){e=e||window.event;e.preventDefault();var content=document.querySelector(".draggable");var width=content.offsetWidth;var height=content.offsetHeight;pos1=(e.clientX-width)-content.offsetLeft;pos2=(e.clientY-height)-content.offsetTop;content.style.width=width+pos1+'px';content.style.height=height+pos2+'px'}function elementDrag(e){e=e||window.event;e.preventDefault();pos1=pos3-e.clientX;pos2=pos4-e.clientY;pos3=e.clientX;pos4=e.clientY;elmnt.style.top=(elmnt.offsetTop-pos2)+"px";elmnt.style.left=(elmnt.offsetLeft-pos1)+"px"}function closeDragElement(){document.onmouseup=null;document.onmousemove=null}}


on('#ajax', 'click', '.music_list_item', function() {
  track = this.parentElement.parentElement.parentElement;
  if (track.classList.contains('gradient-border')) {
    if (track.classList.contains('pause')) {
      music_player.play();
      track.classList.remove('pause');
      return;
    }
    else {
      music_player.pause();
      track.classList.add('pause');
      return;
    }
  }

  counter = music_player.getTrackId();
  list_pk = track.getAttribute('playlist-pk');

  current_type = "lis" + list_pk;
  if ($playlist.getAttribute("data-type") != current_type) {
      save_playlist('/users/progs/save_playlist/' + current_type + "/", counter);
      $playlist.setAttribute("data-type", current_type);
      show_play_items(document.body.querySelector("#ajax"));
  } else {
      music_player.loadPlaylist(0);
      if (FWDMSP.LOAD_PLAYLIST_COMPLETE) {
        setTimeout(function() {
          music_player.playSpecificTrack(0, counter);
        }, 50);
      }
  }
});

function save_playlist(post_link, counter) {
  var playlist_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  playlist_link.open( 'POST', post_link, true );
  playlist_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  playlist_link.onreadystatechange = function () {
    if ( playlist_link.readyState == 4 && playlist_link.status == 200 ) {
      data = JSON.parse(playlist_link.responseText).tracks.reverse();
      for(i=0; i < data.length; i++) {
        title = data[i].title;
        if (title.indexOf(".") != -1) {
          title = title.split(".")[0];
        }
        _source=data[i].url;
        _title=title;
        _thumbPath=data[i].image;
                //_duration=list[i].getAttribute("data-duration");
                //time = msToTime(_duration);
        music_player.addTrack(_source, title, _thumbPath, null, true, false, null) // 4 позиция - время (time)
      }
      music_player.loadPlaylist(0);
      if (FWDMSP.LOAD_PLAYLIST_COMPLETE){
        setTimeout(function() {music_player.playSpecificTrack(0, counter)}, 50);
      }
  }};
  playlist_link.send( null );
};

function add_html_tracks_in_player (html) {
  span = document.createElement("div");
  span.innerHTML = html;
  tracks = span.querySelectorAll(".li");
  for (i=0; i < tracks.length; i++) {
    _source = tracks[i].getAttribute("data-file");
    _title = tracks[i].querySelector(".new_title").innerHTML;
    _thumbPath = "/static/images/news_small3.jpg";
    music_player.addTrack(_source, title, _thumbPath, null, true, false, null)
  }
};
