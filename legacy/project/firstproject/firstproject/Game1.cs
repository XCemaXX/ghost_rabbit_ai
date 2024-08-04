using System;
using System.Collections.Generic;
using System.Linq;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Audio;
using Microsoft.Xna.Framework.Content;
using Microsoft.Xna.Framework.GamerServices;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;
using Microsoft.Xna.Framework.Media;



using System.IO;

namespace firstproject
{
    /// <summary>
    /// This is the main type for your game
    /// </summary>
    public class Game1 : Microsoft.Xna.Framework.Game
    {
                //объявление объектов

        GraphicsDeviceManager graphics;         //настройка и конфигурация видео адаптера
        SpriteBatch spriteBatch;                //класс необходимый для вывода графических изображений на экране

        public KeyboardState keyboardState;     //переменная для хранения состояния клавиатуры
        KeyboardState old;                      //переменная для хранения предыдущего состояния клавиатуры
        MouseState mouseState;                  //хранение состояния мыши
        MouseState prevState;                   //хранение предыдущего состояния мыши
        Random rand = new Random();             //класс для создания произвольных чисел

        //звуки
        Cue cue;                                //переменная для хранения состояния фоновой музыки

        //строки
        string time;                            //Временное хранение предыдущего имени

        //текстуры
        private Texture2D background1;          //текстура фона игры
        Texture2D[] monster= new Texture2D[4];  //массив текстур для монстров
        private Texture2D textureWin;           //Текстура для сообщения о попадении в таблицу рекордов
        private Texture2D GameOver;             //Текстура для GameOver
        private Texture2D TexturePause;         //Текстура для паузы
        private Texture2D TextureRecord;        //Текстура для рекорда

        //класс Sprite
        Sprite[] pols = new Sprite[maxfloor];   //массив класса для платформ с движением
        Sprite Wow1;                            //класс для +1000 с анимацией
        Sprite monstr;                          //класс для монстра с движением
        Sprite homa;                            //класс для главного персонажа игры с анимацией с движением
        Sprite mouse;                           //класс для отрисовки мыши с движением
        SpriteFont font;                        //переменная для загрузки шрифта

        //булевые переменные
        private bool paused = false;            //игра на паузе или нет
        private bool pauseKeyDown = false;      //проверка нажатия паузы
        private bool GO;                        //Окончена игра или нет
        bool win;                               //Попал в рекорды или нет
        bool trololo = false;                   //идет ввод имени игрока или нет
        bool Shift = false;                     //нажат Shift или нет

        //числовые переменные
        int a;                                  //уменьшение ожидания появления монстров, согласно набранным очкам
        double raznica;                         //разница между следующей позицией персонажа и текущей
        int distancey;                          //расстояние по высоте между платформами
        int distancex;                          //расстояние по ширене между платформами
        int points = 0;                         //количество очков
        int nomer = 0;                          //номер текстуры для монстра
        double totalelapsed = 0;                //время между нажатием клавиш
        int MoM;                                //Задержка для +1000
        int zader;                              //Задержка вывода сообщения GameOver
        int zaderwin;                           //Задержка вывода сообщения, что игрок попал в таблицу рекордов
        int frame;                              //время между появлением монстра
        private int menuState;                  //какое меню включено
        private int cursorState;                //над какой последней кнопкой был курсор в главном меню
        private int cursorRec;                  //над какой последней кнопкой был курсор в меню рекордов
        const int maxfloor = 10;                //количество платформ на экране максимальное

        Menu menu;                              //класс главного меню
        About about;                            //класс меню О... и как играть
        Records records;                        //класс меню таблица рекордов
        Options options;                        //класс меню опций   
        
        //переменные взаимодействия объектов друг с другом
        public BoundingBox bbhoma;                              //для главного персонажа игры
        public BoundingBox bbmonstr;                            //для монстров
        public BoundingBox[] bb = new BoundingBox[maxfloor];    //массив для платформ
        public BoundingBox bbMouse;                             //для мышки
        public BoundingBox bbCursorGame;                        //для кнопки Новая игра
        public BoundingBox bbCursorOption;                      //для кнопки опции
        public BoundingBox bbCursorRecord;                      //для кнопки рекорды
        public BoundingBox bbCursorAbout;                       //для кнопки О...
        public BoundingBox bbCursorExit;                        //для кнопки выход или назад
        public BoundingBox bbCursorHTP;                         //для кнопки как играть
        public BoundingBox bbCursorMusic;                       //для кнопки вкл\откл музыку
        public BoundingBox bbCursorSounds;                      //для кнопки вкл\откл звуки
        public BoundingBox bbCursorMode;                        //для кнопки выбора мода игры
        public BoundingBox bbCurcorName;                        //для ввода имени
        

        public Game1()
        {           //создание объектов

            graphics = new GraphicsDeviceManager(this);         //настройка и конфигурация видео адаптера 
            Content = new ContentManager(Services);             //загрузка графической состовляющей  
            Window.Title = "Ghost_Rabbit";                      //изменение заголовка окна
            graphics.PreferredBackBufferWidth = 420;            //расширение экрана
            graphics.PreferredBackBufferHeight = 640;
            graphics.PreferMultiSampling = false;               //отключение MultiSampling

            homa = new Sprite(18, 15.7);                        //главный герой с анимацией: кол-во кадров в секунду 15.7, кол-во кадров 18
            monstr = new Sprite();                              //монстр
            Wow1 = new Sprite(2, 8);                            //+1000 очков с анимацией: кол-во кадров в секунду 8, кол-во кадров 2
            Wow1.spritePosition.X = 120;                        //позиция +1000 на экране
            Wow1.spritePosition.Y = 170;           
            for (int i = 0; i < maxfloor; i++)                  //платформы
                pols[i] = new Sprite();
            mouse = new Sprite();                               //курсор мыши

            //меню
            menu = new Menu();
            about = new About();
            records = new Records();
            options = new Options();
            
            //объекты взаимодействия
            bbMouse = new BoundingBox();
            bbCursorGame = new BoundingBox();
            bbCursorOption = new BoundingBox();
            bbCursorRecord = new BoundingBox();
            bbCursorAbout = new BoundingBox();
            bbCursorExit = new BoundingBox();
            bbCursorHTP = new BoundingBox();
            bbCursorMusic = new BoundingBox();
            bbCursorSounds = new BoundingBox();
            bbCursorMode = new BoundingBox();
            bbCurcorName = new BoundingBox();                                 
          
        }

        /// <summary>
        /// Allows the game to perform any initialization it needs to before starting to run.
        /// This is where it can query for any required services and load any non-graphic
        /// related content.  Calling base.Initialize will enumerate through any components
        /// and initialize them as well.
        /// </summary>
        /// 
        protected override void Initialize()
        {
            menuState = 1;                      //включение главного меню
            cursorState = 1;                    //выделение новой игры
            cursorRec = 0;                      //начальное значение, чтобы при первом входе в меню рекорды была выделена кнопка назад
            options.Initialize();               //функция класса опций: открытие файла с настройками
            Sound.Initialize();                 //инициализация звуков
            cue = Sound.soundbank.GetCue("fon");//загрузка фоновой музыки
            cue.Play();                         //включение фоновой музыки
            if (!options.music) cue.Pause();    //если музыка отключена, то остановить её
            base.Initialize();
           
        }

        /// <summary>
        /// LoadContent will be called once per game and is the place to load
        /// all of your content.
        /// </summary>
        protected override void LoadContent()
        {
            spriteBatch = new SpriteBatch(graphics.GraphicsDevice);                     //создание объекта SpriteBatch
            
                                //загрузка текстур
            homa.Load(Content, "Content\\Textures\\homik1");                            //главный персонаж
            background1 = Content.Load<Texture2D>("Content\\Textures\\background1");    //фон игры
            font = Content.Load<SpriteFont>("Content\\Fonts\\GameFont");                //шрифт
            
            for (int i = 0; i < maxfloor; i++)                                          //платформы согласно уровню сложности
               if (options.mode==1) pols[i].Load(Content, "Content\\Textures\\floor");
               else if (options.mode == 2) pols[i].Load(Content, "Content\\Textures\\floor3");
               else pols[i].Load(Content, "Content\\Textures\\floor2");

            monster[0] = Content.Load<Texture2D>("Content\\Textures\\monstr4");         //монстры
            monster[1] = Content.Load<Texture2D>("Content\\Textures\\monstr1");
            monster[2] = Content.Load<Texture2D>("Content\\Textures\\monstr2");
            monster[3] = Content.Load<Texture2D>("Content\\Textures\\monstr3");
            monstr.Load(Content, "Content\\Textures\\monstr2");                         //загрузка текстуры монстра для класса Sprite, чтобы вычислять ее размеры
            GameOver = Content.Load<Texture2D>("Content\\Textures\\go");                //Game Over
            TexturePause = Content.Load<Texture2D>("Content\\Textures\\pause");         //Пауза
            Wow1.Load(Content, "Content\\Textures\\10001");                             //+1000
            textureWin = Content.Load<Texture2D>("Content\\Textures\\Win");             //Попал в таблицу рекордов
            TextureRecord = Content.Load<Texture2D>("Content\\Textures\\record");       //рекорд

            mouse.Load(Content, "Content\\Textures\\mouse");//мышь
            menu.Load(Content);                 //главное меню            
            about.Load(Content);                //меню О... и Как играть
            records.Load(Content);              //меню Рекордов
            options.Load(Content);              //меню Опций
        }

        /// <summary>
        /// UnloadContent will be called once per game and is the place to unload
        /// all content.
        /// </summary>
        protected override void UnloadContent()
        {
            // TODO: Unload any non ContentManager content here
        }

        /// <summary>
        /// Allows the game to run logic such as updating the world,
        /// checking for collisions, gathering input, and playing audio.
        /// </summary>
        /// <param name="gameTime">Provides a snapshot of timing values.</param>
        protected override void Update(GameTime gameTime)
        {
            old = keyboardState;                 
            keyboardState = Keyboard.GetState();                //считывание состояния клавиатуры
            if (trololo) UpdateInput(gameTime);                 //если вводится имя, то выполняем функцию по вводу имени

                                //проверка на попадение в таблицу рекордов
            if ((options.mode == 3) && (GO) && (zader == 0) && (points > Convert.ToInt32(records.s[9]))) //для сложного уровня
            {
                string b;                           //вспомогательная переменная для записи в таблицу
                records.s[8] = options.name;        //вставка нового рекорда
                records.s[9] = points.ToString();
                for (int i = 7; i > 0; i = i - 2)   //проверка и вставка нового рекорда на более высшие места
                {
                    if (points < Convert.ToInt32(records.s[i])) break;
                    b = records.s[i];
                    records.s[i] = points.ToString();
                    records.s[i + 2] = b;
                    b = records.s[i - 1];
                    records.s[i - 1] = options.name;
                    records.s[i + 1] = b;
                }
                using (StreamWriter sw = File.CreateText("Content\\Text\\player.txt"))  //запись рекордов в файл
                {
                    foreach (string str in records.s)
                        sw.WriteLine(str);
                }
                win = true;                //выводить сообщение о попадении в таблицу
            }

            if ((options.mode == 2) && (GO) && (zader == 0) && (points > Convert.ToInt32(records.s[19])))//для среднего уровня
            {
                string b;                           //вспомогательная переменная для записи в таблицу
                records.s[18] = options.name;       //вставка нового рекорда
                records.s[19] = points.ToString();
                for (int i = 17; i > 10; i = i - 2) //проверка и вставка нового рекорда на более высшие места
                {
                    if (points < Convert.ToInt32(records.s[i])) break;
                    b = records.s[i];
                    records.s[i] = points.ToString();
                    records.s[i + 2] = b;
                    b = records.s[i-1];
                    records.s[i-1] = options.name;
                    records.s[i + 1] = b;
                }
                using (StreamWriter sw = File.CreateText("Content\\Text\\player.txt"))      //запись рекордов в файл
                {
                    foreach (string str in records.s)
                         sw.WriteLine(str);
                }
                win = true;         //выводить сообщение о попадении в таблицу
             }

            if ((zader == 1) && (options.sounds)) 
                Sound.PlayCue(soundList.dead);      //проигрывать звук конца игры, если они вкл. и игра окончена

            if (GO && menuState == 0)               //увеличивать счетчик задержки надписи GameOver и попадения в таблицу
            { 
                zader++; 
                zaderwin++; 
            }

            if (keyboardState.IsKeyDown(Keys.Escape)||(zader==100))     //Выход в меню после проигрыша или нажатия Esc
            {
                if (menuState == 2)                   //При выходе из опций с помощью Esc, ввод останавливается, настройки не изменяются
                { 
                    trololo = false;
                    options.Initialize();
                } 
                if (!win) menuState = 1;                  //Если не попал в таблицу или включено какое-либо меню, то переход в главное
                else { menuState = 3; win = false; }      //Если попал в таблицу, то в меню рекордов
                zader = 0; GO = false; zaderwin = 0;      //обнуление счетчиков задержек и значения проигрыша, после выхода из игрового процесса
            
            }

            if (menuState != 0)  UpdateMouse();           //если не игровой процесс, то вызов функции перехода по меню        
            else
            {
                Pause();                 //функция вкл/Откл паузы в игре
                if (paused == false)     //если пауза не включена, то запускать игровой процесс
                {
                    double elapsed = gameTime.ElapsedGameTime.TotalSeconds;     //текущее значение времени, прошедшее за один игровой цикл
                    homa.UpdateFrame(elapsed);                                  //вызов обновления анимации для главного персонажа
                    Wow1.UpdateFrame(elapsed);                                  //вызов обновления анимации для +1000
                    MoveHoma();                             //движение главного персонажа
                    Collisions();                           //столкновение объектов
                    Updatefloors();                         //движение платформ
                    MoveMonstr();                           //движение монстров
                    if ((points % 1000 < 17) && (points > 17))  MoM = 0;                //при проходе через 1000 счетчик становится равен 0
                    if ((MoM == 1) && (options.sounds)) Sound.PlayCue(soundList.thous); //если начал увеличиваться, то включаем звук
                    if (MoM > -1) MoM++;                                                //увеличиваем счетчик, только если очки кратны 1000
                    if (MoM > 70) MoM = -1;                                             //если задержка максимально, то перестаем показывать
                }    
            }
            Sound.Update();
            base.Update(gameTime);
        }

        /// <summary>
        /// Движение платформ
        /// </summary>
        public void Updatefloors()
        {
            foreach (Sprite pol in pols)
            {
                if (pol.alive)                          //для активной платформы обрабатывается её движение
                {
                    if ((raznica < -20 + Window.ClientBounds.Height / 2) & (homa.xSprite.Y > 0)) //если главный герой на середине экрана, то платформы смещаем вниз, согласно скорости главного героя
                        pol.spritePosition.Y += homa.xSprite.Y;
                    if (pol.spritePosition.Y > Window.ClientBounds.Height)      //при достижении низа экрана - платформа неактивна
                        pol.alive = false;
                }
                else                                //для неактивной платформы обрабатывается её появление
                {
                    distancey = 1000;
                    distancex = 1000;
                    foreach (Sprite poly in pols)
                        if ((poly.alive) & (distancey > poly.spritePosition.Y))     //поиск минимального высоты расположения активной платформы
                        {
                            distancey = (int)poly.spritePosition.Y;
                            distancex = (int)poly.spritePosition.X;
                        }
                    pol.alive = true;                                               //оживление платформы
                    int i = rand.Next(2, Window.ClientBounds.Width - 68);           //ищем первоначальное расположение по ширине
                    if ((i > (distancex + 70) | (i < distancex - 70)) & distancey < 0)//если платформа не накладывается на поставленную платформу, то ставим её
                        pol.spritePosition = new Vector2(i, rand.Next((-250 + distancey), distancey));
                    else if (distancey < 0)                     //если платформа накрывает платформу поставленную, то ставим новую выше
                        pol.spritePosition = new Vector2(i, rand.Next((-250 + distancey), distancey - 25));
                    else                                        //если поставленная платформа находится на экране, то новую ставим выше экрана
                        pol.spritePosition = new Vector2(i, rand.Next((-250 + distancey), -20));
                }
            }
        }

        /// <summary>
        /// Движение главного героя
        /// </summary>
        public void MoveHoma()
        {

            if (keyboardState.IsKeyDown(Keys.Left))         //двигаем влево, если зажато лево
                homa.spritePosition.X -= 7;
            else if (keyboardState.IsKeyDown(Keys.Right))   //двигаем вправо, если зажато право
                homa.spritePosition.X += 7;
            
            if (homa.spritePosition.X < -35)                //если у края экрана, то перемещаемся на другой край
                homa.spritePosition.X = Window.ClientBounds.Width - 35;
            else if (homa.spritePosition.X > Window.ClientBounds.Width - 35)
                homa.spritePosition.X = -35;

            raznica = homa.spritePosition.Y - homa.xSprite.Y;   //вычисляем следующее положение главного героя

            if (raznica >= (-20 + Window.ClientBounds.Height / 2))  //если следующее положение главного героя больше середины экрана, то перемещаем его в это положение
                homa.spritePosition.Y -= homa.xSprite.Y;
            else                                                    //в противном случае, оставляем его на месте
            {
                homa.spritePosition.Y = -20 + Window.ClientBounds.Height / 2;
                points = points + (-(int)raznica - 20 + Window.ClientBounds.Height / 2) / 4;
            }

            homa.xSprite.Y = homa.xSprite.Y - (homa.aSprite.Y) / 2;  //уменьшаем скорость подъема главного героя на величину ускорения

            if (homa.spritePosition.Y > Window.ClientBounds.Height) //если главный герой вышел за нижний край экрана
            {
               
                if (options.mode != 1)  //если мод не практика, то Игра окончена
                    GO = true;
                else                    //в практике восстанавливаем главного героя на середине экрана
                { 
                    homa.spritePosition.Y = 320;
                    homa.spritePosition.X = 210;
                    homa.xSprite.Y = 0;
                }

            }

        }


        /// <summary>
        /// Движение монстров
        /// </summary>
        public void MoveMonstr()
        {            
            if (!monstr.alive) frame++;         //если монстр не активен, то увеличиваем ожидание
            if (frame == 150-a)                 //если ожидание максимально, то делаем монстра активным и задаем его движение
            {
                monstr.alive = true;
                nomer = rand.Next(0, 4);        //выбираем произвольно текстуру для монстра
                float e;
                                                                                    //задание скорости монстру
                if (options.mode == 3) e = (float)rand.NextDouble() * 3 + 4;        //в сложном режиме монстры двигаются быстрее
                else e = (float)rand.NextDouble() * 4 + 2;

                int ohoho = rand.Next(1, 4);                        //задаем сторону появления монстра
                if (ohoho == 1)                 //поялвение монстра сверху
                {
                    monstr.spritePosition.X = rand.Next(-monstr.spriteTexture.Width, Window.ClientBounds.Width);
                    monstr.spritePosition.Y = -70;
                }
                else if (ohoho == 2)            //поялвение монстра слева
                {
                    monstr.spritePosition.X = -56;
                    monstr.spritePosition.Y = rand.Next(0, Window.ClientBounds.Height / 2 - 150);
                }
                else                            //поялвение монстра справа
                {
                    monstr.spritePosition.X = Window.ClientBounds.Width;
                    monstr.spritePosition.Y = rand.Next(0, Window.ClientBounds.Height / 2 - 150);
                }
                monstr.xSprite.X = e * (homa.spritePosition.X - monstr.spritePosition.X) / (monstr.spriteTexture.Width + Window.ClientBounds.Width);
                monstr.xSprite.Y = (int)Math.Sqrt(e * e - monstr.xSprite.X * monstr.xSprite.X); //высчитывание скорости монстра по x и по y

                if ((points < 150*150)&&(options.mode!=3)) a = (int)Math.Sqrt(points);      //уменьшение ожидания согласно очкам, но при наборе очков больше 22500, монстры непрерывны, как и в сложном режиме
                else a = 149;

                frame = 0;                      //обнуляем ожидание после появления монстра
            }

            monstr.spritePosition.X = monstr.spritePosition.X + monstr.xSprite.X;
            monstr.spritePosition.Y = monstr.spritePosition.Y + monstr.xSprite.Y;           //перемещение монстра согласно скоростям
            if ((monstr.spritePosition.Y > Window.ClientBounds.Height + 10) || (monstr.spritePosition.X > Window.ClientBounds.Width + 100) || (monstr.spritePosition.X < -100)) monstr.alive = false; //монстр не активен
            if ((raznica < -20 + Window.ClientBounds.Height / 2) & (homa.xSprite.Y > 0))
                monstr.spritePosition.Y += homa.xSprite.Y;              //учитывается движение главного героя, если он на середине экрана, то опускаем все
        }

        /// <summary>
        /// Пауза в игре
        /// </summary>
        public void Pause()
        {
            if (keyboardState.IsKeyDown(Keys.P))//если кнопку p нажали, то кнопка p нажата
            {
                pauseKeyDown = true;
            }
            else if (pauseKeyDown)  //если кнопка p нажата, то вкл/откл паузу и отжимаем кнопку
            {
                pauseKeyDown = false;
                paused = !paused;
            }
        }


        /// <summary>
        /// Взаимодействие объектов в игровом процессе
        /// </summary>
        public void Collisions()
        {                   //задаем координаты полей взаимойдействий
            bbhoma.Min = new Vector3(homa.spritePosition.X + 25, homa.spritePosition.Y + homa.spriteTexture.Height-31, 0);     //для главного героя
            bbhoma.Max = new Vector3(homa.spritePosition.X + 25, homa.spritePosition.Y + homa.spriteTexture.Height-31, 0);      //-31, т.к. голова поднята на 31 пиксель

            if (monstr.alive)           //для монстра
            {
                bbmonstr.Min = new Vector3(monstr.spritePosition.X + 5, monstr.spritePosition.Y + 5, 0);
                bbmonstr.Max = new Vector3(monstr.spritePosition.X + monstr.spriteTexture.Width - 5, monstr.spritePosition.Y + monstr.spriteTexture.Height + 50, 0);
            }
            else
            {
                bbmonstr.Min = new Vector3(0, 0, 0);
                bbmonstr.Max = new Vector3(0, 0, 0);
            }

            for (int i = 0; bb.Length > i; i++)             //для платформ
            {
                bb[i].Min = new Vector3(pols[i].spritePosition.X,
                pols[i].spritePosition.Y, 0);
                bb[i].Max = new Vector3(pols[i].spritePosition.X + pols[i].spriteTexture.Width,
                    pols[i].spritePosition.Y + pols[i].spriteTexture.Height, 0);
            }

            for (int i = 0; bb.Length > i; i++)         //если главный герой взаимодейстует с платформой и двигается вниз, то меняем его скорость на максимальную и проигрывается звук удара
                if (bbhoma.Intersects(bb[i]) && homa.xSprite.Y < 0)
                {
                    if (options.sounds) Sound.PlayCue(soundList.hit);
                    homa.xSprite.Y = 16;
                    
                }
            if (bbhoma.Intersects(bbmonstr)) {  homa.spritePosition.Y = Window.ClientBounds.Height * 2; } //если пересекатся главный герой с монстром, то убиваем его(перемещаем за экран)
        }


        /// <summary>
        /// преднастроки перед новой игрой
        /// </summary>
        public void NewGame()
        {
            homa.spritePosition = new Vector2(Window.ClientBounds.Width / 2, Window.ClientBounds.Height - 95);      //начальная позиция главного героя

            if (options.sounds) Sound.PlayCue(soundList.Start);         //проигрываем звук при старте

            MoM = -1;                            //обнуляем значение задержки для +1000
            paused = false;                      //снимаем игру с паузы, если при прошлом выходе она была на паузе
            monstr.alive = false;                //монстра делаем неактивным
            frame = 0; a = 0;                    //задержку для монстра и уменьшение задержки из-за очков обнуляем

            pols[0].alive = true;               //делаем активным первую платформу, остальные неактивные
            pols[0].spritePosition = new Vector2(Window.ClientBounds.Width / 2 - 25, Window.ClientBounds.Height - 40);
            for (int i = 1; i < pols.Length; i++)
                pols[i].alive = false;

            distancey = Window.ClientBounds.Height - 40;                    //находим минимальное расстояние между активной платформой и будующей
            for (int i = 1; (i < pols.Length) & (distancey > 0); i++)       //ставим начальные платформы
            {
                pols[i].alive = true;
                distancey = (int)pols[i - 1].spritePosition.Y;
                pols[i].spritePosition = new Vector2(rand.Next(2, Window.ClientBounds.Width - 68), rand.Next((-250 + distancey), distancey - 25));
            }

            points = 0;                             //обнуляем очки
            homa.xSprite.Y = 16;                    //устанавливаем максимальную скорость главному герою
            base.Initialize();
        }


        /// <summary>
        /// Обрабатываем нажатые клавиши
        /// </summary>
        /// <param name="key"></param>
        /// <returns></returns>
        public String GetText(Keys key)
        {
            
            String text = key.ToString();
            if (text.Contains("NumPad"))                    //проверка на нажатие NumPad
            { Shift = false; return text.Remove(0, 6); }
            else if (text.Length == 2 && text.Contains("D"))//проверка на нажатие цифр
            { Shift = false; return text.Remove(0, 1); }
            else if ((text.Length == 1)&&(Shift))           //проверка на нажатие буквы с Shift
            { Shift = false; return text.ToUpper(); }
            else if (text.Length == 1)                      //проверка на нажатие буквы без Shift
                return text.ToLower();
            else
                return "";
           
        }

        /// <summary>
        /// Ввод с клавиатуры
        /// </summary>
        /// <param name="gameTime"></param>
        public void UpdateInput(GameTime gameTime)
        {
            double elapsed = gameTime.ElapsedGameTime.TotalSeconds;      //текущее значение времени, прошедшее за один игровой цикл
            totalelapsed += elapsed;                                     //увеличиваем время между нажатием клавиш   
          
            Keys[] pressedKeys;                                         //массив нажатых клавиш
            pressedKeys = keyboardState.GetPressedKeys();               //заполняем массив нажатых клавиш, соответственно нажатым клавишам

            if (keyboardState.IsKeyUp(Keys.RightShift) && keyboardState.IsKeyUp(Keys.LeftShift)) Shift = false;//проверка на зажатие Shift     
            foreach (Keys key in pressedKeys)              //для каждой начатой клавиши
            {
                if (options.name.Length <= 7)              //если длина имени больше 7, то ввод прекращается
                {
                    if (old != keyboardState) totalelapsed = 0.13;      //если была нажата другая кнопка(или вообще ничего не нажато), то сокращаем время задержки
                        if (key.ToString() == "Enter")     //если нажат Enter, то ввод прекращается
                        { 
                            trololo = false;
                            if (options.name != "_")        //Если ввод был, то сохраняем имя         
                            { options.name = options.name.Remove(options.name.Length - 1, 1); time = options.name; }
                            else options.name = time;       //Если не введено символов, то возращаем прежнее имя
                        }
                        else if (key.ToString().Contains("Shift"))                //проверка на нажатие Shift
                        Shift = true;
                        else if (totalelapsed > 0.15)      //новый одинаковый символ вводится, после задержки, чтобы на одно нажатие клавиши не приходилось несколько символов
                        {
                            options.name = options.name.Remove(options.name.Length - 1, 1) + GetText(key) + "_";
                            totalelapsed =0;
                            if ((options.name.Length > 1) && ((key.ToString() == "Back")||(key.ToString() == "Delete"))) //удаление символа
                                options.name = options.name.Remove(options.name.Length - 2, 1);
                        }
                        
                }
                else
                {
                    options.name = options.name.Remove(options.name.Length - 1, 1); //сохраняем имя, прекращаем ввод
                    time = options.name;
                    trololo = false; 
                }
            }
        }
            
        /// <summary>
        /// обновление состояния мыши в меню
        /// </summary>
        public void UpdateMouse()
        {            
            prevState = mouseState;
            mouseState = Mouse.GetState();                  //считываем предыдущее состояние мыши и текущее

            mouse.spritePosition.X = mouseState.X;          //определяем позицию курсора
            mouse.spritePosition.Y = mouseState.Y;

            //задаем координаты полей взаимодействий
            bbMouse.Min = new Vector3(mouse.spritePosition.X, mouse.spritePosition.Y, 0);      //для курсора 
            bbMouse.Max = new Vector3(mouse.spritePosition.X + mouse.spriteTexture.Width, mouse.spritePosition.Y + mouse.spriteTexture.Height, 0);

            if (menuState == 1)             //в главном меню
            {
                bbCursorExit.Min = new Vector3(304, 577, 0);   //для кнопки Выход
                bbCursorExit.Max = new Vector3(358, 610, 0);

                bbCursorRecord.Min = new Vector3(229, 424, 0);  //для кнопки Рекорды
                bbCursorRecord.Max = new Vector3(356, 458, 0);

                bbCursorGame.Min = new Vector3(23, 295, 0);         //для кнопки новой игры
                bbCursorGame.Max = new Vector3(162, 331, 0);

                bbCursorOption.Min = new Vector3(142, 359, 0);      //для кнопки опций
                bbCursorOption.Max = new Vector3(255, 390, 0);

                bbCursorAbout.Min = new Vector3(314, 497, 0);       //для кнопки О...
                bbCursorAbout.Max = new Vector3(402, 532, 0);

                bbCursorHTP.Min = new Vector3(108, 528, 0);         //для кнопки Как играть
                bbCursorHTP.Max = new Vector3(171, 611, 0);
            }
            else if (menuState == 2)                        //в меню опции
            {
                bbCurcorName.Min = new Vector3(220, 157, 0);        //для ввода имени
                bbCurcorName.Max = new Vector3(340, 200, 0);

                bbCursorMode.Min = new Vector3(220, 370,0);         //для выбора уровня сложности
                bbCursorMode.Max = new Vector3(339,407,0);

                bbCursorMusic.Min = new Vector3(300, 215,0);        //для кнопки Музыка
                bbCursorMusic.Max = new Vector3(300 + options.cursorButton.Width, 215 + options.cursorButton.Height, 0);

                bbCursorSounds.Min = new Vector3(300, 285,0);       //для кнопки звуки
                bbCursorSounds.Max = new Vector3(300 + options.cursorButton.Width, 285 + options.cursorButton.Height, 0);

                bbCursorOption.Min = new Vector3(260, 470, 0);      //для кнопки не сохранять опции
                bbCursorOption.Max = new Vector3(260 + 105, 470 + 35, 0);

                bbCursorExit.Min = new Vector3(204, 577, 0);        //для кнопки назад
                bbCursorExit.Max = new Vector3(258, 610, 0);
            }
            else if (menuState == 3)            //для меню рекорды
            {
                bbCursorExit.Min = new Vector3(204, 577, 0);        //для кнопки назад
                bbCursorExit.Max = new Vector3(258, 610, 0);

                bbCursorRecord.Min = new Vector3(280, 490, 0);      //для кнопки обновить таблицу рекордов
                bbCursorRecord.Max = new Vector3(280 + 130, 490 + 68, 0);
            }
            else                   //для меню Как играть, О...
            {
                bbCursorExit.Min = new Vector3(204, 577, 0);        //для кнопки назад
                bbCursorExit.Max = new Vector3(258, 610, 0);
            }

                        //пересечение курсора(не нажатый) и кнопок
            if (menuState == 1)   //в главном меню 
            {   
                if (bbMouse.Intersects(bbCursorGame))    //c новой игрой
                    cursorState = 1;   
                if (bbMouse.Intersects(bbCursorOption))    //с опциями
                    cursorState = 3;    
                if (bbMouse.Intersects(bbCursorRecord))    //с рекордами
                    cursorState = 4;    
                if (bbMouse.Intersects(bbCursorAbout))    // с O...
                    cursorState = 5;   
                if (bbMouse.Intersects(bbCursorHTP))    //с Как играть
                    cursorState = 6;    
                if (bbMouse.Intersects(bbCursorExit))    //c Выход
                    cursorState = 2;    
            }
            else if (menuState == 3)//в меню рекорды
            {
                if (bbMouse.Intersects(bbCursorRecord))//с Обновить талицу рекордов
                        cursorRec = 1;
                if (bbMouse.Intersects(bbCursorExit))//с Назад
                        cursorRec = 0;
            }
            else if (menuState == 2)//в меню опции
            {
                if (bbMouse.Intersects(bbCursorOption))//с Обновить талицу рекордов
                    cursorRec = 1;
                if (bbMouse.Intersects(bbCursorExit))//с Назад
                    cursorRec = 0;
            }
            //нажатие по кнопкам
            if (menuState == 1)         //в главном меню
            {
                if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorGame) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter)&&(cursorState==1)))
                {//если курсор находится над кнопкой, и левая кнопка нажата, и прошлое состояние мыши - не нажата, или нажата клавиша Enter и кнопка выделена, то включаем игру
                    menuState = 0;
                    this.NewGame();
                }
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorOption) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 3)))
                { 
                    menuState = 2;
                }//аналогично опции
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorRecord) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 4)))
                {
                    menuState = 3;
                }//аналогично рекорды
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorAbout) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 5)))
                {
                    menuState = 4;
                }//аналогично О...
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorHTP) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 6)))
                {
                    menuState = 5;
                }//аналогично Как играть
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorExit) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 2)))
                    this.Exit();//аналогично Выход
            }
            else if (menuState==3)          //в меню рекорды
            {
                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorRecord) && prevState.LeftButton != ButtonState.Pressed)
                {
                    records.s = File.ReadAllLines("Content\\Text\\records.txt");
                    if (File.Exists("Content\\Text\\player.txt"))
                        System.IO.File.Delete("Content\\Text\\player.txt");
                }                           //при нажатии на обновить таблицу, из файла records.txt загружается таблица
                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorExit) && prevState.LeftButton != ButtonState.Pressed) 
                    menuState = 1;          //при нажатии назад, выход в главное меню
            }
            else if (menuState==2)              //в меню опции
            {

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCurcorName) && prevState.LeftButton != ButtonState.Pressed)
                {
                    if (!trololo) time = options.name;      //при новом нажатии на имя(ввод уже был), оно стирается, но не запоминается
                    trololo = true;                         //но если ввода не было то оно запомнится
                    options.name = "_";         //при нажатии на на имя, начинается ввод имени
                }

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorMusic) && prevState.LeftButton != ButtonState.Pressed)
                    options.music = !options.music;             //вкл/откл музыки

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorSounds) && prevState.LeftButton != ButtonState.Pressed)
                    options.sounds = !options.sounds;           //вкл/откл звуков

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorMode) && prevState.LeftButton != ButtonState.Pressed)
                {
                    options.mode++;
                    if (options.mode > 3) options.mode = 1;     //выбор уровня сложности
                }

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorExit) && prevState.LeftButton != ButtonState.Pressed)
                {               //сохраняем настройки при выходе из меню опций
                    if (trololo) //прекращаем ввод имени если нажата кнопка выхода
                    { 
                        trololo = false;
                        options.name = time; //При выходе, имя не сохраняется, а возращается прежнее
                       
                    }

                    if (options.music && cue.IsPaused) cue.Resume();    //проверяем изменились ли настройки музыки, если да, то pause/play

                    if (!options.music && cue.IsPlaying) cue.Pause();
                    
                           //сохраняем настройки в файл
                    using (StreamWriter sw = File.CreateText("Content\\Text\\option.txt"))
                        {
                            sw.WriteLine(options.name);
                            sw.WriteLine(options.mode.ToString());
                            sw.WriteLine(options.music.ToString());
                            sw.WriteLine(options.sounds.ToString());
                        }
                    menuState = 1;          //открываем главное меню
                }
                else if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorOption) && prevState.LeftButton != ButtonState.Pressed)
                {               //не сохраняем настройки при выходе из меню опций
                        trololo = false;//прекращаем ввод имени если нажата кнопка выхода
                        options.name = time; //При выходе, имя не сохраняется, а возращается прежнее
                        options.Initialize();
                        menuState = 1;          //открываем главное меню
                }
            }
            else if ((menuState==4)||(menuState==5))//обработка нажатия кнопки назад в меню О... и Как играть
            { 
                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorExit) && prevState.LeftButton != ButtonState.Pressed) 
                menuState = 1; 
            }           
        }

        /// <summary>
        /// This is called when the game should draw itself.
        /// </summary>
        /// <param name="gameTime">Provides a snapshot of timing values.</param>
       
        protected override void Draw(GameTime gameTime)
        {
            GraphicsDevice.Clear(Color.CornflowerBlue);             //очищаем экран
            if (menuState == 1)         //отрисовывыем главное меню
            {
                spriteBatch.Begin();
                menu.DrawMenu(spriteBatch, cursorState);
                mouse.DrawSprite(spriteBatch);
                spriteBatch.End();
            }
            else if ((menuState == 4) || (menuState == 5))      //отрисовывыем О... или Как играть
            {
                spriteBatch.Begin();
                about.DrawAbout(spriteBatch, menuState);
                mouse.DrawSprite(spriteBatch);
                spriteBatch.End();           
            }
            else if (menuState == 3)                //Отрисовываем Рекорды
            {
                spriteBatch.Begin();
                records.DrawRecord(spriteBatch, cursorRec);
                mouse.DrawSprite(spriteBatch);
                spriteBatch.End(); 
            }
            else if (menuState == 2)            //Отрисовываем опции
            {
                spriteBatch.Begin();
                options.DrawOption(spriteBatch, cursorRec);
                mouse.DrawSprite(spriteBatch);
                spriteBatch.End();
            }
            else if (menuState == 0)            //отрисовываем игру
            {
                spriteBatch.Begin();
              
                spriteBatch.Draw(background1, new Vector2(0, 0), Color.White);          //задний фон
               
                foreach (Sprite pol in pols)                    //если платформа активна, то отрисовываем
                {
                    if (pol.alive)
                    { pol.DrawSprite(spriteBatch); }
                }

                homa.DrawAnimationSprite(spriteBatch);          //отрисовка главного персонажа
               
                if (monstr.alive)                               //Если монстр существует, то отрисовываем его
                {
                    spriteBatch.Draw(monster[nomer], monstr.spritePosition, Color.White);
                }

                spriteBatch.DrawString(font,"Score: " + points.ToString(),new Vector2(270, 20),Color.Yellow);   //Вывод текущего числа очков
               
                spriteBatch.DrawString(font,options.name,new Vector2(40, 20),Color.Yellow);         //Вывод имени игрока
               
                if (MoM != -1) Wow1.DrawAnimationSprite(spriteBatch); //рисует анимацию, когда игрок набирает очередную тысячу
               
                if (GO)                 //если игрок проиграл, то выводится GameOver
                {
                    if ((win) && (zaderwin > 40)) 
                        spriteBatch.Draw(textureWin, new Vector2(100, 200), Color.White);         //Если попал в таблицу рекордов, то после GameOver выводится сообщение
                    else
                        spriteBatch.Draw(GameOver, new Vector2(70, 280), Color.White);          //Вывод GameOver
                }
                if(options.mode==3)     //отрисовка надписи рекорд во время игры, чтобы показать пользователю что он побил предыдущий рекорд
                    for (int i=1;i<10;i=i+2)                    //для сложного уровня
                        spriteBatch.Draw(TextureRecord, new Vector2(0, (points - Convert.ToInt32(records.s[i])) * 4 + Window.ClientBounds.Height / 2), Color.White);
                else if (options.mode == 2)
                    for (int i = 11; i < 20; i = i + 2)         //для среднего уровня
                        spriteBatch.Draw(TextureRecord, new Vector2(0, (points - Convert.ToInt32(records.s[i])) * 4 + Window.ClientBounds.Height / 2), Color.White);                
                if (paused&&!GO) spriteBatch.Draw(TexturePause, new Vector2(70, 280), Color.White);   //Отрисовка паузы, если игра не окончена и стоит на паузе
     
                spriteBatch.End();
            }

            base.Draw(gameTime);
        }

    }
}
