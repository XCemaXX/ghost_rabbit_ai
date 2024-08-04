using System;
using System.Collections.Generic;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Content;
using Microsoft.Xna.Framework.Graphics;
using System.IO;



namespace firstproject
{
    class Records
    {
        private Texture2D recordTexture;        //Текстура фона меню рекорды
        private Vector2 recordPosition;         //Расположение заднего фона

        private Texture2D cursorClear;          //Текстура неактивной кнопки очистить
        private Texture2D cursorClear2;         //Текстура активной кнопки очистить
        private Vector2 cursorPositionClear;    //Расположение кнопки очистить

        private Texture2D cursorExit;           //Текстура неактивной кнопки назад
        private Texture2D cursorExit2;          //Текстура активной кнопки назад
        private Vector2 cursorPositionExit;     //Расположение кнопки назад

        public string[] s;              //таблица рекордов в массиве строк
        
        SpriteFont font;                //Шрифт

        /// <summary>
        /// начальные значения
        /// </summary>
        public Records()
        {
            recordPosition = new Vector2(0, 0);
            cursorPositionExit = new Vector2(204, 577);
            cursorPositionClear = new Vector2(280,490);
        }

        /// <summary>
        /// Загрузка текстур и таблицы рекордов
        /// </summary>
        /// <param name="content"></param>
        public void Load(ContentManager content)
        {
            recordTexture = content.Load<Texture2D>("Content\\Textures\\bg_rec");   //Текстура фон
            cursorClear = content.Load<Texture2D>("Content\\Textures\\clear1");     //Текстура неактивной кнопки очистить
            cursorClear2 = content.Load<Texture2D>("Content\\Textures\\clear2");    //Текстура активной кнопки очистить
            cursorExit = content.Load<Texture2D>("Content\\Textures\\back1");       //Текстура неактивной кнопки назад
            cursorExit2 = content.Load<Texture2D>("Content\\Textures\\back2");      //Текстура активной кнопки назад

            if (!File.Exists("Content\\Text\\records.txt"))                                 //если файла records.txt нет, 
                using (StreamWriter sw = File.CreateText("Content\\Text\\records.txt"))     //то создаем и заполняем его дефолтными значениями
                {
                    for (int i = 0; i < 20; i=i+2)
                    {
                        sw.WriteLine("Player");
                        sw.WriteLine("0");
                    }
                }

            if (!File.Exists("Content\\Text\\player.txt"))              //если файла с пользовательскими рекордами нет, то 
            {                                                           //считываем с файла records.txt и записываем значения в пользовательский
                s = File.ReadAllLines("Content\\Text\\records.txt");
                using (StreamWriter sw = File.CreateText("Content\\Text\\player.txt"))
                {
                    foreach (string str in s)
                    sw.WriteLine(str);
                }
            }
            else s = File.ReadAllLines("Content\\Text\\player.txt");        //если файла с пользовательскими рекордами есть, то считываем с него
            
            font = content.Load<SpriteFont>("Content\\Fonts\\GameFont");    //Загружаем шрифт
        }

        /// <summary>
        /// Отрисовка меню рекорды
        /// </summary>
        /// <param name="spriteBatch"></param>
        /// <param name="x"></param>
        public void DrawRecord(SpriteBatch spriteBatch, int x)
        {
            spriteBatch.Draw(recordTexture, recordPosition, Color.White);
            if (x == 0)             //Если активна кнопка Назад, то рисуем её активной, а очистеть нет
            {
                spriteBatch.Draw(cursorExit2, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorClear, cursorPositionClear, Color.White);
            }
            else                    //в противном случае - наоборот
            {
                spriteBatch.Draw(cursorExit, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorClear2, cursorPositionClear, Color.White);
            }
            
            for (int i = 0; i < 10;i++ )        //Рисуем таблицу рекордов, для сложного уровня
            {
                if (i % 2 == 0)                 //Четная строка - Имя, нечетная - очки
                    spriteBatch.DrawString(font, ((i/2)+1).ToString() +". "+ s[i], new Vector2(50, 120 + i * 15), Color.Red);
                else
                    spriteBatch.DrawString(font, s[i], new Vector2(300, 120 + (i - 1) * 15), Color.Red);
            }

            for (int i = 10; i < 20; i++)       //Рисуем таблицу рекордов, для среднего уровня
            {
                if (i % 2 == 0)                 //Четная строка - Имя, нечетная - очки
                    spriteBatch.DrawString(font, ((i / 2) + 1-5).ToString() + ". " + s[i], new Vector2(50, 160 + i * 15), Color.Yellow);
                else
                    spriteBatch.DrawString(font, s[i], new Vector2(300, 160 + (i - 1) * 15), Color.Yellow);
            }
        }
    }
}
