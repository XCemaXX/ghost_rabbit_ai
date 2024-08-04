using System;
using System.Collections.Generic;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Content;
using Microsoft.Xna.Framework.Graphics;
using System.IO;

namespace firstproject
{
    class Options
    {
        private Texture2D optionTexture;        //Текстура заднего фона опций
        private Vector2 optionPosition;         //Расположение заднего фона
        private Texture2D cursorExit;           //Текстура неактивной кнопки назад
        private Texture2D cursorExit2;          //Текстура активной кнопки назад
        private Vector2 cursorPositionExit;     //Расположение кнопки Назад
        private Texture2D cursorCancel;         //Текстура неактивной кнопки Сбросить настройки
        private Texture2D cursorCancel2;        //Текстура активной кнопки Сбросить настройки
        private Vector2 cursorPositionCancel;   //Расположение кнопки Сбросить настройки
        public Texture2D cursorButton;          //Текстура вкл
        private Texture2D cursorButton2;        //Текстура выкл
        private Vector2 cursorPositionMusic;    //Расположение выключателя музыки
        private Vector2 cursorPositionSounds;   //Расположение выключателя звуков
        private Vector2 cursorPositionMode;     //Расположение уровня сложности
        private Texture2D cursorHard;           //Текстура сложного уровня
        private Texture2D cursorNorm;           //Текстура среднего уровня
        private Texture2D cursorPrac;           //Текстура легкого уровня
        
        public string name;             //Имя игрока
        public bool music;              //Значение музыки
        public bool sounds;             //Значение звуков
        public int mode;                //Значение уровня сложности
        
        SpriteFont font;                //Шрифт

        /// <summary>
        /// начальные значения
        /// </summary>
        public Options()
        {
            optionPosition = new Vector2(0, 0);
            cursorPositionExit = new Vector2(204, 577);
            cursorPositionCancel = new Vector2(260, 470);
            cursorPositionMusic= new Vector2(300,215);

            cursorPositionSounds = new Vector2(300, 285);

            cursorPositionMode = new Vector2(220, 370);
        }

        /// <summary>
        /// Открытие файла options.txt
        /// </summary>
        public void Initialize()
        {
            string[] set;
            if (File.Exists("content\\Text\\option.txt"))           //Если файл есть, то считываем с него значения настроек
            {
                set = File.ReadAllLines("content\\Text\\option.txt");
                name = set[0];
                if (set[1] == "1") mode = 1;
                else if (set[1] == "2") mode = 2;
                else mode = 3;
                if (set[2] == "False") music = false;
                else music = true;
                if (set[3] == "False") sounds = false;
                else sounds = true;
            }
            else                        //в противном случае записать дефолтные настройки в файл и придать эти настройки переменным
            {
                mode = 2; music = true; sounds = true; name = "Player";
                using (StreamWriter sw = File.CreateText("Content\\Text\\option.txt"))
                {
                    sw.WriteLine(name);
                    sw.WriteLine(mode.ToString());
                    sw.WriteLine(music.ToString());
                    sw.WriteLine(sounds.ToString());
                }
            }
                   
        }

        /// <summary>
        /// Загрузка текстур для опций
        /// </summary>
        /// <param name="content"></param>
        public void Load(ContentManager content)
        {
            optionTexture = content.Load<Texture2D>("Content\\Textures\\bg_opt");       //текстура фон
            
            cursorExit = content.Load<Texture2D>("Content\\Textures\\save1");           //Текстура неактивной кнопки назад
            cursorExit2 = content.Load<Texture2D>("Content\\Textures\\save2");          //Текстура активной кнопки назад

            cursorCancel = content.Load<Texture2D>("Content\\Textures\\cancel1");       //Текстура неактивной кнопки Сбросить настройки
            cursorCancel2 = content.Load<Texture2D>("Content\\Textures\\cancel2");      //Текстура активной кнопки Сбросить настройки

            cursorButton = content.Load<Texture2D>("Content\\Textures\\butt_on");       //текстура включенной кнопки
            cursorButton2 = content.Load<Texture2D>("Content\\Textures\\butt_off");     //текстура выключенной кнопки

            cursorHard = content.Load<Texture2D>("Content\\Textures\\mod_unreal");      //текстура сложного уровня
            cursorNorm = content.Load<Texture2D>("Content\\Textures\\mod_normal");      //текстура среднего уровня
            cursorPrac = content.Load<Texture2D>("Content\\Textures\\mod_practice");    //текстура легкого уровня
            font = content.Load<SpriteFont>("Content\\Fonts\\GameFont");                //текстура шрифта          
        }

        /// <summary>
        /// отрисовка опций
        /// </summary>
        /// <param name="spriteBatch"></param>
        public void DrawOption(SpriteBatch spriteBatch, int x)
        {   spriteBatch.Draw(optionTexture, optionPosition, Color.White);

            if (mode == 3) spriteBatch.Draw(cursorHard, cursorPositionMode, Color.White);       //Отрисовка уровня сложности
            else if (mode == 2) spriteBatch.Draw(cursorNorm, cursorPositionMode, Color.White);
            else spriteBatch.Draw(cursorPrac, cursorPositionMode, Color.White);

            if (music) spriteBatch.Draw(cursorButton, cursorPositionMusic, Color.White);        //Отрисовка музыки вкл/выкл
            else spriteBatch.Draw(cursorButton2, cursorPositionMusic, Color.White);

            if (sounds) spriteBatch.Draw(cursorButton, cursorPositionSounds, Color.White);      //Отрисовка звука вкл/выкл
            else spriteBatch.Draw(cursorButton2, cursorPositionSounds, Color.White);


            if (x == 0)             //Если активна кнопка Назад, то рисуем её активной, а не сохранять опции нет
            {
                spriteBatch.Draw(cursorExit2, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorCancel, cursorPositionCancel, Color.White);
            }
            else                    //в противном случае - наоборот
            {
                spriteBatch.Draw(cursorExit, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorCancel2, cursorPositionCancel, Color.White);
            }

            spriteBatch.DrawString(font, name, new Vector2(220, 157), Color.White);             //Отрисовка имени                      
        }
    }
}