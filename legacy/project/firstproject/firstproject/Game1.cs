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
                //���������� ��������

        GraphicsDeviceManager graphics;         //��������� � ������������ ����� ��������
        SpriteBatch spriteBatch;                //����� ����������� ��� ������ ����������� ����������� �� ������

        public KeyboardState keyboardState;     //���������� ��� �������� ��������� ����������
        KeyboardState old;                      //���������� ��� �������� ����������� ��������� ����������
        MouseState mouseState;                  //�������� ��������� ����
        MouseState prevState;                   //�������� ����������� ��������� ����
        Random rand = new Random();             //����� ��� �������� ������������ �����

        //�����
        Cue cue;                                //���������� ��� �������� ��������� ������� ������

        //������
        string time;                            //��������� �������� ����������� �����

        //��������
        private Texture2D background1;          //�������� ���� ����
        Texture2D[] monster= new Texture2D[4];  //������ ������� ��� ��������
        private Texture2D textureWin;           //�������� ��� ��������� � ��������� � ������� ��������
        private Texture2D GameOver;             //�������� ��� GameOver
        private Texture2D TexturePause;         //�������� ��� �����
        private Texture2D TextureRecord;        //�������� ��� �������

        //����� Sprite
        Sprite[] pols = new Sprite[maxfloor];   //������ ������ ��� �������� � ���������
        Sprite Wow1;                            //����� ��� +1000 � ���������
        Sprite monstr;                          //����� ��� ������� � ���������
        Sprite homa;                            //����� ��� �������� ��������� ���� � ��������� � ���������
        Sprite mouse;                           //����� ��� ��������� ���� � ���������
        SpriteFont font;                        //���������� ��� �������� ������

        //������� ����������
        private bool paused = false;            //���� �� ����� ��� ���
        private bool pauseKeyDown = false;      //�������� ������� �����
        private bool GO;                        //�������� ���� ��� ���
        bool win;                               //����� � ������� ��� ���
        bool trololo = false;                   //���� ���� ����� ������ ��� ���
        bool Shift = false;                     //����� Shift ��� ���

        //�������� ����������
        int a;                                  //���������� �������� ��������� ��������, �������� ��������� �����
        double raznica;                         //������� ����� ��������� �������� ��������� � �������
        int distancey;                          //���������� �� ������ ����� �����������
        int distancex;                          //���������� �� ������ ����� �����������
        int points = 0;                         //���������� �����
        int nomer = 0;                          //����� �������� ��� �������
        double totalelapsed = 0;                //����� ����� �������� ������
        int MoM;                                //�������� ��� +1000
        int zader;                              //�������� ������ ��������� GameOver
        int zaderwin;                           //�������� ������ ���������, ��� ����� ����� � ������� ��������
        int frame;                              //����� ����� ���������� �������
        private int menuState;                  //����� ���� ��������
        private int cursorState;                //��� ����� ��������� ������� ��� ������ � ������� ����
        private int cursorRec;                  //��� ����� ��������� ������� ��� ������ � ���� ��������
        const int maxfloor = 10;                //���������� �������� �� ������ ������������

        Menu menu;                              //����� �������� ����
        About about;                            //����� ���� �... � ��� ������
        Records records;                        //����� ���� ������� ��������
        Options options;                        //����� ���� �����   
        
        //���������� �������������� �������� ���� � ������
        public BoundingBox bbhoma;                              //��� �������� ��������� ����
        public BoundingBox bbmonstr;                            //��� ��������
        public BoundingBox[] bb = new BoundingBox[maxfloor];    //������ ��� ��������
        public BoundingBox bbMouse;                             //��� �����
        public BoundingBox bbCursorGame;                        //��� ������ ����� ����
        public BoundingBox bbCursorOption;                      //��� ������ �����
        public BoundingBox bbCursorRecord;                      //��� ������ �������
        public BoundingBox bbCursorAbout;                       //��� ������ �...
        public BoundingBox bbCursorExit;                        //��� ������ ����� ��� �����
        public BoundingBox bbCursorHTP;                         //��� ������ ��� ������
        public BoundingBox bbCursorMusic;                       //��� ������ ���\���� ������
        public BoundingBox bbCursorSounds;                      //��� ������ ���\���� �����
        public BoundingBox bbCursorMode;                        //��� ������ ������ ���� ����
        public BoundingBox bbCurcorName;                        //��� ����� �����
        

        public Game1()
        {           //�������� ��������

            graphics = new GraphicsDeviceManager(this);         //��������� � ������������ ����� �������� 
            Content = new ContentManager(Services);             //�������� ����������� ������������  
            Window.Title = "Ghost_Rabbit";                      //��������� ��������� ����
            graphics.PreferredBackBufferWidth = 420;            //���������� ������
            graphics.PreferredBackBufferHeight = 640;
            graphics.PreferMultiSampling = false;               //���������� MultiSampling

            homa = new Sprite(18, 15.7);                        //������� ����� � ���������: ���-�� ������ � ������� 15.7, ���-�� ������ 18
            monstr = new Sprite();                              //������
            Wow1 = new Sprite(2, 8);                            //+1000 ����� � ���������: ���-�� ������ � ������� 8, ���-�� ������ 2
            Wow1.spritePosition.X = 120;                        //������� +1000 �� ������
            Wow1.spritePosition.Y = 170;           
            for (int i = 0; i < maxfloor; i++)                  //���������
                pols[i] = new Sprite();
            mouse = new Sprite();                               //������ ����

            //����
            menu = new Menu();
            about = new About();
            records = new Records();
            options = new Options();
            
            //������� ��������������
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
            menuState = 1;                      //��������� �������� ����
            cursorState = 1;                    //��������� ����� ����
            cursorRec = 0;                      //��������� ��������, ����� ��� ������ ����� � ���� ������� ���� �������� ������ �����
            options.Initialize();               //������� ������ �����: �������� ����� � �����������
            Sound.Initialize();                 //������������� ������
            cue = Sound.soundbank.GetCue("fon");//�������� ������� ������
            cue.Play();                         //��������� ������� ������
            if (!options.music) cue.Pause();    //���� ������ ���������, �� ���������� �
            base.Initialize();
           
        }

        /// <summary>
        /// LoadContent will be called once per game and is the place to load
        /// all of your content.
        /// </summary>
        protected override void LoadContent()
        {
            spriteBatch = new SpriteBatch(graphics.GraphicsDevice);                     //�������� ������� SpriteBatch
            
                                //�������� �������
            homa.Load(Content, "Content\\Textures\\homik1");                            //������� ��������
            background1 = Content.Load<Texture2D>("Content\\Textures\\background1");    //��� ����
            font = Content.Load<SpriteFont>("Content\\Fonts\\GameFont");                //�����
            
            for (int i = 0; i < maxfloor; i++)                                          //��������� �������� ������ ���������
               if (options.mode==1) pols[i].Load(Content, "Content\\Textures\\floor");
               else if (options.mode == 2) pols[i].Load(Content, "Content\\Textures\\floor3");
               else pols[i].Load(Content, "Content\\Textures\\floor2");

            monster[0] = Content.Load<Texture2D>("Content\\Textures\\monstr4");         //�������
            monster[1] = Content.Load<Texture2D>("Content\\Textures\\monstr1");
            monster[2] = Content.Load<Texture2D>("Content\\Textures\\monstr2");
            monster[3] = Content.Load<Texture2D>("Content\\Textures\\monstr3");
            monstr.Load(Content, "Content\\Textures\\monstr2");                         //�������� �������� ������� ��� ������ Sprite, ����� ��������� �� �������
            GameOver = Content.Load<Texture2D>("Content\\Textures\\go");                //Game Over
            TexturePause = Content.Load<Texture2D>("Content\\Textures\\pause");         //�����
            Wow1.Load(Content, "Content\\Textures\\10001");                             //+1000
            textureWin = Content.Load<Texture2D>("Content\\Textures\\Win");             //����� � ������� ��������
            TextureRecord = Content.Load<Texture2D>("Content\\Textures\\record");       //������

            mouse.Load(Content, "Content\\Textures\\mouse");//����
            menu.Load(Content);                 //������� ����            
            about.Load(Content);                //���� �... � ��� ������
            records.Load(Content);              //���� ��������
            options.Load(Content);              //���� �����
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
            keyboardState = Keyboard.GetState();                //���������� ��������� ����������
            if (trololo) UpdateInput(gameTime);                 //���� �������� ���, �� ��������� ������� �� ����� �����

                                //�������� �� ��������� � ������� ��������
            if ((options.mode == 3) && (GO) && (zader == 0) && (points > Convert.ToInt32(records.s[9]))) //��� �������� ������
            {
                string b;                           //��������������� ���������� ��� ������ � �������
                records.s[8] = options.name;        //������� ������ �������
                records.s[9] = points.ToString();
                for (int i = 7; i > 0; i = i - 2)   //�������� � ������� ������ ������� �� ����� ������ �����
                {
                    if (points < Convert.ToInt32(records.s[i])) break;
                    b = records.s[i];
                    records.s[i] = points.ToString();
                    records.s[i + 2] = b;
                    b = records.s[i - 1];
                    records.s[i - 1] = options.name;
                    records.s[i + 1] = b;
                }
                using (StreamWriter sw = File.CreateText("Content\\Text\\player.txt"))  //������ �������� � ����
                {
                    foreach (string str in records.s)
                        sw.WriteLine(str);
                }
                win = true;                //�������� ��������� � ��������� � �������
            }

            if ((options.mode == 2) && (GO) && (zader == 0) && (points > Convert.ToInt32(records.s[19])))//��� �������� ������
            {
                string b;                           //��������������� ���������� ��� ������ � �������
                records.s[18] = options.name;       //������� ������ �������
                records.s[19] = points.ToString();
                for (int i = 17; i > 10; i = i - 2) //�������� � ������� ������ ������� �� ����� ������ �����
                {
                    if (points < Convert.ToInt32(records.s[i])) break;
                    b = records.s[i];
                    records.s[i] = points.ToString();
                    records.s[i + 2] = b;
                    b = records.s[i-1];
                    records.s[i-1] = options.name;
                    records.s[i + 1] = b;
                }
                using (StreamWriter sw = File.CreateText("Content\\Text\\player.txt"))      //������ �������� � ����
                {
                    foreach (string str in records.s)
                         sw.WriteLine(str);
                }
                win = true;         //�������� ��������� � ��������� � �������
             }

            if ((zader == 1) && (options.sounds)) 
                Sound.PlayCue(soundList.dead);      //����������� ���� ����� ����, ���� ��� ���. � ���� ��������

            if (GO && menuState == 0)               //����������� ������� �������� ������� GameOver � ��������� � �������
            { 
                zader++; 
                zaderwin++; 
            }

            if (keyboardState.IsKeyDown(Keys.Escape)||(zader==100))     //����� � ���� ����� ��������� ��� ������� Esc
            {
                if (menuState == 2)                   //��� ������ �� ����� � ������� Esc, ���� ���������������, ��������� �� ����������
                { 
                    trololo = false;
                    options.Initialize();
                } 
                if (!win) menuState = 1;                  //���� �� ����� � ������� ��� �������� �����-���� ����, �� ������� � �������
                else { menuState = 3; win = false; }      //���� ����� � �������, �� � ���� ��������
                zader = 0; GO = false; zaderwin = 0;      //��������� ��������� �������� � �������� ���������, ����� ������ �� �������� ��������
            
            }

            if (menuState != 0)  UpdateMouse();           //���� �� ������� �������, �� ����� ������� �������� �� ����        
            else
            {
                Pause();                 //������� ���/���� ����� � ����
                if (paused == false)     //���� ����� �� ��������, �� ��������� ������� �������
                {
                    double elapsed = gameTime.ElapsedGameTime.TotalSeconds;     //������� �������� �������, ��������� �� ���� ������� ����
                    homa.UpdateFrame(elapsed);                                  //����� ���������� �������� ��� �������� ���������
                    Wow1.UpdateFrame(elapsed);                                  //����� ���������� �������� ��� +1000
                    MoveHoma();                             //�������� �������� ���������
                    Collisions();                           //������������ ��������
                    Updatefloors();                         //�������� ��������
                    MoveMonstr();                           //�������� ��������
                    if ((points % 1000 < 17) && (points > 17))  MoM = 0;                //��� ������� ����� 1000 ������� ���������� ����� 0
                    if ((MoM == 1) && (options.sounds)) Sound.PlayCue(soundList.thous); //���� ����� �������������, �� �������� ����
                    if (MoM > -1) MoM++;                                                //����������� �������, ������ ���� ���� ������ 1000
                    if (MoM > 70) MoM = -1;                                             //���� �������� �����������, �� ��������� ����������
                }    
            }
            Sound.Update();
            base.Update(gameTime);
        }

        /// <summary>
        /// �������� ��������
        /// </summary>
        public void Updatefloors()
        {
            foreach (Sprite pol in pols)
            {
                if (pol.alive)                          //��� �������� ��������� �������������� � ��������
                {
                    if ((raznica < -20 + Window.ClientBounds.Height / 2) & (homa.xSprite.Y > 0)) //���� ������� ����� �� �������� ������, �� ��������� ������� ����, �������� �������� �������� �����
                        pol.spritePosition.Y += homa.xSprite.Y;
                    if (pol.spritePosition.Y > Window.ClientBounds.Height)      //��� ���������� ���� ������ - ��������� ���������
                        pol.alive = false;
                }
                else                                //��� ���������� ��������� �������������� � ���������
                {
                    distancey = 1000;
                    distancex = 1000;
                    foreach (Sprite poly in pols)
                        if ((poly.alive) & (distancey > poly.spritePosition.Y))     //����� ������������ ������ ������������ �������� ���������
                        {
                            distancey = (int)poly.spritePosition.Y;
                            distancex = (int)poly.spritePosition.X;
                        }
                    pol.alive = true;                                               //��������� ���������
                    int i = rand.Next(2, Window.ClientBounds.Width - 68);           //���� �������������� ������������ �� ������
                    if ((i > (distancex + 70) | (i < distancex - 70)) & distancey < 0)//���� ��������� �� ������������� �� ������������ ���������, �� ������ �
                        pol.spritePosition = new Vector2(i, rand.Next((-250 + distancey), distancey));
                    else if (distancey < 0)                     //���� ��������� ��������� ��������� ������������, �� ������ ����� ����
                        pol.spritePosition = new Vector2(i, rand.Next((-250 + distancey), distancey - 25));
                    else                                        //���� ������������ ��������� ��������� �� ������, �� ����� ������ ���� ������
                        pol.spritePosition = new Vector2(i, rand.Next((-250 + distancey), -20));
                }
            }
        }

        /// <summary>
        /// �������� �������� �����
        /// </summary>
        public void MoveHoma()
        {

            if (keyboardState.IsKeyDown(Keys.Left))         //������� �����, ���� ������ ����
                homa.spritePosition.X -= 7;
            else if (keyboardState.IsKeyDown(Keys.Right))   //������� ������, ���� ������ �����
                homa.spritePosition.X += 7;
            
            if (homa.spritePosition.X < -35)                //���� � ���� ������, �� ������������ �� ������ ����
                homa.spritePosition.X = Window.ClientBounds.Width - 35;
            else if (homa.spritePosition.X > Window.ClientBounds.Width - 35)
                homa.spritePosition.X = -35;

            raznica = homa.spritePosition.Y - homa.xSprite.Y;   //��������� ��������� ��������� �������� �����

            if (raznica >= (-20 + Window.ClientBounds.Height / 2))  //���� ��������� ��������� �������� ����� ������ �������� ������, �� ���������� ��� � ��� ���������
                homa.spritePosition.Y -= homa.xSprite.Y;
            else                                                    //� ��������� ������, ��������� ��� �� �����
            {
                homa.spritePosition.Y = -20 + Window.ClientBounds.Height / 2;
                points = points + (-(int)raznica - 20 + Window.ClientBounds.Height / 2) / 4;
            }

            homa.xSprite.Y = homa.xSprite.Y - (homa.aSprite.Y) / 2;  //��������� �������� ������� �������� ����� �� �������� ���������

            if (homa.spritePosition.Y > Window.ClientBounds.Height) //���� ������� ����� ����� �� ������ ���� ������
            {
               
                if (options.mode != 1)  //���� ��� �� ��������, �� ���� ��������
                    GO = true;
                else                    //� �������� ��������������� �������� ����� �� �������� ������
                { 
                    homa.spritePosition.Y = 320;
                    homa.spritePosition.X = 210;
                    homa.xSprite.Y = 0;
                }

            }

        }


        /// <summary>
        /// �������� ��������
        /// </summary>
        public void MoveMonstr()
        {            
            if (!monstr.alive) frame++;         //���� ������ �� �������, �� ����������� ��������
            if (frame == 150-a)                 //���� �������� �����������, �� ������ ������� �������� � ������ ��� ��������
            {
                monstr.alive = true;
                nomer = rand.Next(0, 4);        //�������� ����������� �������� ��� �������
                float e;
                                                                                    //������� �������� �������
                if (options.mode == 3) e = (float)rand.NextDouble() * 3 + 4;        //� ������� ������ ������� ��������� �������
                else e = (float)rand.NextDouble() * 4 + 2;

                int ohoho = rand.Next(1, 4);                        //������ ������� ��������� �������
                if (ohoho == 1)                 //��������� ������� ������
                {
                    monstr.spritePosition.X = rand.Next(-monstr.spriteTexture.Width, Window.ClientBounds.Width);
                    monstr.spritePosition.Y = -70;
                }
                else if (ohoho == 2)            //��������� ������� �����
                {
                    monstr.spritePosition.X = -56;
                    monstr.spritePosition.Y = rand.Next(0, Window.ClientBounds.Height / 2 - 150);
                }
                else                            //��������� ������� ������
                {
                    monstr.spritePosition.X = Window.ClientBounds.Width;
                    monstr.spritePosition.Y = rand.Next(0, Window.ClientBounds.Height / 2 - 150);
                }
                monstr.xSprite.X = e * (homa.spritePosition.X - monstr.spritePosition.X) / (monstr.spriteTexture.Width + Window.ClientBounds.Width);
                monstr.xSprite.Y = (int)Math.Sqrt(e * e - monstr.xSprite.X * monstr.xSprite.X); //������������ �������� ������� �� x � �� y

                if ((points < 150*150)&&(options.mode!=3)) a = (int)Math.Sqrt(points);      //���������� �������� �������� �����, �� ��� ������ ����� ������ 22500, ������� ����������, ��� � � ������� ������
                else a = 149;

                frame = 0;                      //�������� �������� ����� ��������� �������
            }

            monstr.spritePosition.X = monstr.spritePosition.X + monstr.xSprite.X;
            monstr.spritePosition.Y = monstr.spritePosition.Y + monstr.xSprite.Y;           //����������� ������� �������� ���������
            if ((monstr.spritePosition.Y > Window.ClientBounds.Height + 10) || (monstr.spritePosition.X > Window.ClientBounds.Width + 100) || (monstr.spritePosition.X < -100)) monstr.alive = false; //������ �� �������
            if ((raznica < -20 + Window.ClientBounds.Height / 2) & (homa.xSprite.Y > 0))
                monstr.spritePosition.Y += homa.xSprite.Y;              //����������� �������� �������� �����, ���� �� �� �������� ������, �� �������� ���
        }

        /// <summary>
        /// ����� � ����
        /// </summary>
        public void Pause()
        {
            if (keyboardState.IsKeyDown(Keys.P))//���� ������ p ������, �� ������ p ������
            {
                pauseKeyDown = true;
            }
            else if (pauseKeyDown)  //���� ������ p ������, �� ���/���� ����� � �������� ������
            {
                pauseKeyDown = false;
                paused = !paused;
            }
        }


        /// <summary>
        /// �������������� �������� � ������� ��������
        /// </summary>
        public void Collisions()
        {                   //������ ���������� ����� ���������������
            bbhoma.Min = new Vector3(homa.spritePosition.X + 25, homa.spritePosition.Y + homa.spriteTexture.Height-31, 0);     //��� �������� �����
            bbhoma.Max = new Vector3(homa.spritePosition.X + 25, homa.spritePosition.Y + homa.spriteTexture.Height-31, 0);      //-31, �.�. ������ ������� �� 31 �������

            if (monstr.alive)           //��� �������
            {
                bbmonstr.Min = new Vector3(monstr.spritePosition.X + 5, monstr.spritePosition.Y + 5, 0);
                bbmonstr.Max = new Vector3(monstr.spritePosition.X + monstr.spriteTexture.Width - 5, monstr.spritePosition.Y + monstr.spriteTexture.Height + 50, 0);
            }
            else
            {
                bbmonstr.Min = new Vector3(0, 0, 0);
                bbmonstr.Max = new Vector3(0, 0, 0);
            }

            for (int i = 0; bb.Length > i; i++)             //��� ��������
            {
                bb[i].Min = new Vector3(pols[i].spritePosition.X,
                pols[i].spritePosition.Y, 0);
                bb[i].Max = new Vector3(pols[i].spritePosition.X + pols[i].spriteTexture.Width,
                    pols[i].spritePosition.Y + pols[i].spriteTexture.Height, 0);
            }

            for (int i = 0; bb.Length > i; i++)         //���� ������� ����� �������������� � ���������� � ��������� ����, �� ������ ��� �������� �� ������������ � ������������� ���� �����
                if (bbhoma.Intersects(bb[i]) && homa.xSprite.Y < 0)
                {
                    if (options.sounds) Sound.PlayCue(soundList.hit);
                    homa.xSprite.Y = 16;
                    
                }
            if (bbhoma.Intersects(bbmonstr)) {  homa.spritePosition.Y = Window.ClientBounds.Height * 2; } //���� ����������� ������� ����� � ��������, �� ������� ���(���������� �� �����)
        }


        /// <summary>
        /// ������������ ����� ����� �����
        /// </summary>
        public void NewGame()
        {
            homa.spritePosition = new Vector2(Window.ClientBounds.Width / 2, Window.ClientBounds.Height - 95);      //��������� ������� �������� �����

            if (options.sounds) Sound.PlayCue(soundList.Start);         //����������� ���� ��� ������

            MoM = -1;                            //�������� �������� �������� ��� +1000
            paused = false;                      //������� ���� � �����, ���� ��� ������� ������ ��� ���� �� �����
            monstr.alive = false;                //������� ������ ����������
            frame = 0; a = 0;                    //�������� ��� ������� � ���������� �������� ��-�� ����� ��������

            pols[0].alive = true;               //������ �������� ������ ���������, ��������� ����������
            pols[0].spritePosition = new Vector2(Window.ClientBounds.Width / 2 - 25, Window.ClientBounds.Height - 40);
            for (int i = 1; i < pols.Length; i++)
                pols[i].alive = false;

            distancey = Window.ClientBounds.Height - 40;                    //������� ����������� ���������� ����� �������� ���������� � ��������
            for (int i = 1; (i < pols.Length) & (distancey > 0); i++)       //������ ��������� ���������
            {
                pols[i].alive = true;
                distancey = (int)pols[i - 1].spritePosition.Y;
                pols[i].spritePosition = new Vector2(rand.Next(2, Window.ClientBounds.Width - 68), rand.Next((-250 + distancey), distancey - 25));
            }

            points = 0;                             //�������� ����
            homa.xSprite.Y = 16;                    //������������� ������������ �������� �������� �����
            base.Initialize();
        }


        /// <summary>
        /// ������������ ������� �������
        /// </summary>
        /// <param name="key"></param>
        /// <returns></returns>
        public String GetText(Keys key)
        {
            
            String text = key.ToString();
            if (text.Contains("NumPad"))                    //�������� �� ������� NumPad
            { Shift = false; return text.Remove(0, 6); }
            else if (text.Length == 2 && text.Contains("D"))//�������� �� ������� ����
            { Shift = false; return text.Remove(0, 1); }
            else if ((text.Length == 1)&&(Shift))           //�������� �� ������� ����� � Shift
            { Shift = false; return text.ToUpper(); }
            else if (text.Length == 1)                      //�������� �� ������� ����� ��� Shift
                return text.ToLower();
            else
                return "";
           
        }

        /// <summary>
        /// ���� � ����������
        /// </summary>
        /// <param name="gameTime"></param>
        public void UpdateInput(GameTime gameTime)
        {
            double elapsed = gameTime.ElapsedGameTime.TotalSeconds;      //������� �������� �������, ��������� �� ���� ������� ����
            totalelapsed += elapsed;                                     //����������� ����� ����� �������� ������   
          
            Keys[] pressedKeys;                                         //������ ������� ������
            pressedKeys = keyboardState.GetPressedKeys();               //��������� ������ ������� ������, �������������� ������� ��������

            if (keyboardState.IsKeyUp(Keys.RightShift) && keyboardState.IsKeyUp(Keys.LeftShift)) Shift = false;//�������� �� ������� Shift     
            foreach (Keys key in pressedKeys)              //��� ������ ������� �������
            {
                if (options.name.Length <= 7)              //���� ����� ����� ������ 7, �� ���� ������������
                {
                    if (old != keyboardState) totalelapsed = 0.13;      //���� ���� ������ ������ ������(��� ������ ������ �� ������), �� ��������� ����� ��������
                        if (key.ToString() == "Enter")     //���� ����� Enter, �� ���� ������������
                        { 
                            trololo = false;
                            if (options.name != "_")        //���� ���� ���, �� ��������� ���         
                            { options.name = options.name.Remove(options.name.Length - 1, 1); time = options.name; }
                            else options.name = time;       //���� �� ������� ��������, �� ��������� ������� ���
                        }
                        else if (key.ToString().Contains("Shift"))                //�������� �� ������� Shift
                        Shift = true;
                        else if (totalelapsed > 0.15)      //����� ���������� ������ ��������, ����� ��������, ����� �� ���� ������� ������� �� ����������� ��������� ��������
                        {
                            options.name = options.name.Remove(options.name.Length - 1, 1) + GetText(key) + "_";
                            totalelapsed =0;
                            if ((options.name.Length > 1) && ((key.ToString() == "Back")||(key.ToString() == "Delete"))) //�������� �������
                                options.name = options.name.Remove(options.name.Length - 2, 1);
                        }
                        
                }
                else
                {
                    options.name = options.name.Remove(options.name.Length - 1, 1); //��������� ���, ���������� ����
                    time = options.name;
                    trololo = false; 
                }
            }
        }
            
        /// <summary>
        /// ���������� ��������� ���� � ����
        /// </summary>
        public void UpdateMouse()
        {            
            prevState = mouseState;
            mouseState = Mouse.GetState();                  //��������� ���������� ��������� ���� � �������

            mouse.spritePosition.X = mouseState.X;          //���������� ������� �������
            mouse.spritePosition.Y = mouseState.Y;

            //������ ���������� ����� ��������������
            bbMouse.Min = new Vector3(mouse.spritePosition.X, mouse.spritePosition.Y, 0);      //��� ������� 
            bbMouse.Max = new Vector3(mouse.spritePosition.X + mouse.spriteTexture.Width, mouse.spritePosition.Y + mouse.spriteTexture.Height, 0);

            if (menuState == 1)             //� ������� ����
            {
                bbCursorExit.Min = new Vector3(304, 577, 0);   //��� ������ �����
                bbCursorExit.Max = new Vector3(358, 610, 0);

                bbCursorRecord.Min = new Vector3(229, 424, 0);  //��� ������ �������
                bbCursorRecord.Max = new Vector3(356, 458, 0);

                bbCursorGame.Min = new Vector3(23, 295, 0);         //��� ������ ����� ����
                bbCursorGame.Max = new Vector3(162, 331, 0);

                bbCursorOption.Min = new Vector3(142, 359, 0);      //��� ������ �����
                bbCursorOption.Max = new Vector3(255, 390, 0);

                bbCursorAbout.Min = new Vector3(314, 497, 0);       //��� ������ �...
                bbCursorAbout.Max = new Vector3(402, 532, 0);

                bbCursorHTP.Min = new Vector3(108, 528, 0);         //��� ������ ��� ������
                bbCursorHTP.Max = new Vector3(171, 611, 0);
            }
            else if (menuState == 2)                        //� ���� �����
            {
                bbCurcorName.Min = new Vector3(220, 157, 0);        //��� ����� �����
                bbCurcorName.Max = new Vector3(340, 200, 0);

                bbCursorMode.Min = new Vector3(220, 370,0);         //��� ������ ������ ���������
                bbCursorMode.Max = new Vector3(339,407,0);

                bbCursorMusic.Min = new Vector3(300, 215,0);        //��� ������ ������
                bbCursorMusic.Max = new Vector3(300 + options.cursorButton.Width, 215 + options.cursorButton.Height, 0);

                bbCursorSounds.Min = new Vector3(300, 285,0);       //��� ������ �����
                bbCursorSounds.Max = new Vector3(300 + options.cursorButton.Width, 285 + options.cursorButton.Height, 0);

                bbCursorOption.Min = new Vector3(260, 470, 0);      //��� ������ �� ��������� �����
                bbCursorOption.Max = new Vector3(260 + 105, 470 + 35, 0);

                bbCursorExit.Min = new Vector3(204, 577, 0);        //��� ������ �����
                bbCursorExit.Max = new Vector3(258, 610, 0);
            }
            else if (menuState == 3)            //��� ���� �������
            {
                bbCursorExit.Min = new Vector3(204, 577, 0);        //��� ������ �����
                bbCursorExit.Max = new Vector3(258, 610, 0);

                bbCursorRecord.Min = new Vector3(280, 490, 0);      //��� ������ �������� ������� ��������
                bbCursorRecord.Max = new Vector3(280 + 130, 490 + 68, 0);
            }
            else                   //��� ���� ��� ������, �...
            {
                bbCursorExit.Min = new Vector3(204, 577, 0);        //��� ������ �����
                bbCursorExit.Max = new Vector3(258, 610, 0);
            }

                        //����������� �������(�� �������) � ������
            if (menuState == 1)   //� ������� ���� 
            {   
                if (bbMouse.Intersects(bbCursorGame))    //c ����� �����
                    cursorState = 1;   
                if (bbMouse.Intersects(bbCursorOption))    //� �������
                    cursorState = 3;    
                if (bbMouse.Intersects(bbCursorRecord))    //� ���������
                    cursorState = 4;    
                if (bbMouse.Intersects(bbCursorAbout))    // � O...
                    cursorState = 5;   
                if (bbMouse.Intersects(bbCursorHTP))    //� ��� ������
                    cursorState = 6;    
                if (bbMouse.Intersects(bbCursorExit))    //c �����
                    cursorState = 2;    
            }
            else if (menuState == 3)//� ���� �������
            {
                if (bbMouse.Intersects(bbCursorRecord))//� �������� ������ ��������
                        cursorRec = 1;
                if (bbMouse.Intersects(bbCursorExit))//� �����
                        cursorRec = 0;
            }
            else if (menuState == 2)//� ���� �����
            {
                if (bbMouse.Intersects(bbCursorOption))//� �������� ������ ��������
                    cursorRec = 1;
                if (bbMouse.Intersects(bbCursorExit))//� �����
                    cursorRec = 0;
            }
            //������� �� �������
            if (menuState == 1)         //� ������� ����
            {
                if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorGame) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter)&&(cursorState==1)))
                {//���� ������ ��������� ��� �������, � ����� ������ ������, � ������� ��������� ���� - �� ������, ��� ������ ������� Enter � ������ ��������, �� �������� ����
                    menuState = 0;
                    this.NewGame();
                }
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorOption) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 3)))
                { 
                    menuState = 2;
                }//���������� �����
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorRecord) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 4)))
                {
                    menuState = 3;
                }//���������� �������
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorAbout) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 5)))
                {
                    menuState = 4;
                }//���������� �...
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorHTP) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 6)))
                {
                    menuState = 5;
                }//���������� ��� ������
                else if ((mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorExit) && prevState.LeftButton != ButtonState.Pressed) || (keyboardState.IsKeyDown(Keys.Enter) && (cursorState == 2)))
                    this.Exit();//���������� �����
            }
            else if (menuState==3)          //� ���� �������
            {
                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorRecord) && prevState.LeftButton != ButtonState.Pressed)
                {
                    records.s = File.ReadAllLines("Content\\Text\\records.txt");
                    if (File.Exists("Content\\Text\\player.txt"))
                        System.IO.File.Delete("Content\\Text\\player.txt");
                }                           //��� ������� �� �������� �������, �� ����� records.txt ����������� �������
                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorExit) && prevState.LeftButton != ButtonState.Pressed) 
                    menuState = 1;          //��� ������� �����, ����� � ������� ����
            }
            else if (menuState==2)              //� ���� �����
            {

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCurcorName) && prevState.LeftButton != ButtonState.Pressed)
                {
                    if (!trololo) time = options.name;      //��� ����� ������� �� ���(���� ��� ���), ��� ���������, �� �� ������������
                    trololo = true;                         //�� ���� ����� �� ���� �� ��� ����������
                    options.name = "_";         //��� ������� �� �� ���, ���������� ���� �����
                }

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorMusic) && prevState.LeftButton != ButtonState.Pressed)
                    options.music = !options.music;             //���/���� ������

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorSounds) && prevState.LeftButton != ButtonState.Pressed)
                    options.sounds = !options.sounds;           //���/���� ������

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorMode) && prevState.LeftButton != ButtonState.Pressed)
                {
                    options.mode++;
                    if (options.mode > 3) options.mode = 1;     //����� ������ ���������
                }

                if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorExit) && prevState.LeftButton != ButtonState.Pressed)
                {               //��������� ��������� ��� ������ �� ���� �����
                    if (trololo) //���������� ���� ����� ���� ������ ������ ������
                    { 
                        trololo = false;
                        options.name = time; //��� ������, ��� �� �����������, � ����������� �������
                       
                    }

                    if (options.music && cue.IsPaused) cue.Resume();    //��������� ���������� �� ��������� ������, ���� ��, �� pause/play

                    if (!options.music && cue.IsPlaying) cue.Pause();
                    
                           //��������� ��������� � ����
                    using (StreamWriter sw = File.CreateText("Content\\Text\\option.txt"))
                        {
                            sw.WriteLine(options.name);
                            sw.WriteLine(options.mode.ToString());
                            sw.WriteLine(options.music.ToString());
                            sw.WriteLine(options.sounds.ToString());
                        }
                    menuState = 1;          //��������� ������� ����
                }
                else if (mouseState.LeftButton == ButtonState.Pressed && bbMouse.Intersects(bbCursorOption) && prevState.LeftButton != ButtonState.Pressed)
                {               //�� ��������� ��������� ��� ������ �� ���� �����
                        trololo = false;//���������� ���� ����� ���� ������ ������ ������
                        options.name = time; //��� ������, ��� �� �����������, � ����������� �������
                        options.Initialize();
                        menuState = 1;          //��������� ������� ����
                }
            }
            else if ((menuState==4)||(menuState==5))//��������� ������� ������ ����� � ���� �... � ��� ������
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
            GraphicsDevice.Clear(Color.CornflowerBlue);             //������� �����
            if (menuState == 1)         //������������ ������� ����
            {
                spriteBatch.Begin();
                menu.DrawMenu(spriteBatch, cursorState);
                mouse.DrawSprite(spriteBatch);
                spriteBatch.End();
            }
            else if ((menuState == 4) || (menuState == 5))      //������������ �... ��� ��� ������
            {
                spriteBatch.Begin();
                about.DrawAbout(spriteBatch, menuState);
                mouse.DrawSprite(spriteBatch);
                spriteBatch.End();           
            }
            else if (menuState == 3)                //������������ �������
            {
                spriteBatch.Begin();
                records.DrawRecord(spriteBatch, cursorRec);
                mouse.DrawSprite(spriteBatch);
                spriteBatch.End(); 
            }
            else if (menuState == 2)            //������������ �����
            {
                spriteBatch.Begin();
                options.DrawOption(spriteBatch, cursorRec);
                mouse.DrawSprite(spriteBatch);
                spriteBatch.End();
            }
            else if (menuState == 0)            //������������ ����
            {
                spriteBatch.Begin();
              
                spriteBatch.Draw(background1, new Vector2(0, 0), Color.White);          //������ ���
               
                foreach (Sprite pol in pols)                    //���� ��������� �������, �� ������������
                {
                    if (pol.alive)
                    { pol.DrawSprite(spriteBatch); }
                }

                homa.DrawAnimationSprite(spriteBatch);          //��������� �������� ���������
               
                if (monstr.alive)                               //���� ������ ����������, �� ������������ ���
                {
                    spriteBatch.Draw(monster[nomer], monstr.spritePosition, Color.White);
                }

                spriteBatch.DrawString(font,"Score: " + points.ToString(),new Vector2(270, 20),Color.Yellow);   //����� �������� ����� �����
               
                spriteBatch.DrawString(font,options.name,new Vector2(40, 20),Color.Yellow);         //����� ����� ������
               
                if (MoM != -1) Wow1.DrawAnimationSprite(spriteBatch); //������ ��������, ����� ����� �������� ��������� ������
               
                if (GO)                 //���� ����� ��������, �� ��������� GameOver
                {
                    if ((win) && (zaderwin > 40)) 
                        spriteBatch.Draw(textureWin, new Vector2(100, 200), Color.White);         //���� ����� � ������� ��������, �� ����� GameOver ��������� ���������
                    else
                        spriteBatch.Draw(GameOver, new Vector2(70, 280), Color.White);          //����� GameOver
                }
                if(options.mode==3)     //��������� ������� ������ �� ����� ����, ����� �������� ������������ ��� �� ����� ���������� ������
                    for (int i=1;i<10;i=i+2)                    //��� �������� ������
                        spriteBatch.Draw(TextureRecord, new Vector2(0, (points - Convert.ToInt32(records.s[i])) * 4 + Window.ClientBounds.Height / 2), Color.White);
                else if (options.mode == 2)
                    for (int i = 11; i < 20; i = i + 2)         //��� �������� ������
                        spriteBatch.Draw(TextureRecord, new Vector2(0, (points - Convert.ToInt32(records.s[i])) * 4 + Window.ClientBounds.Height / 2), Color.White);                
                if (paused&&!GO) spriteBatch.Draw(TexturePause, new Vector2(70, 280), Color.White);   //��������� �����, ���� ���� �� �������� � ����� �� �����
     
                spriteBatch.End();
            }

            base.Draw(gameTime);
        }

    }
}
