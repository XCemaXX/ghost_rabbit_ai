using System;
using System.Collections.Generic;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Content;
using Microsoft.Xna.Framework.Graphics;

namespace firstproject
{
    class Menu
    {
        private Texture2D menuTexture;          //Текстура фона меню
        private Vector2 menuPosition;           //Расположение заднего фона
        private Texture2D cursorGame;           //Текстура неактивной кнопки Новая игра
        private Texture2D cursorGame2;          //Текстура активной кнопки Новая игра
        private Vector2 cursorPositionGame;     //Расположение кнопки Новая игра
        private Texture2D cursorOption;         //Текстура неактивной кнопки Опции
        private Texture2D cursorOption2;        //Текстура активной кнопки Опции
        private Vector2 cursorPositionOption;   //Расположение кнопки Опции
        private Texture2D cursorRecord;         //Текстура неактивной кнопки Рекорды
        private Texture2D cursorRecord2;        //Текстура активной кнопки Рекорды
        private Vector2 cursorPositionRecord;   //Расположение кнопки Рекорды
        private Texture2D cursorAbout;          //Текстура неактивной кнопки О...
        private Texture2D cursorAbout2;         //Текстура активной кнопки О...
        private Vector2 cursorPositionAbout;    //Расположение кнопки О...
        private Texture2D cursorHTP;            //Текстура неактивной кнопки Как играть
        private Texture2D cursorHTP2;           //Текстура активной кнопки Как играть
        private Vector2 cursorPositionHTP;      //Расположение кнопки Как играть
        private Texture2D cursorExit;           //Текстура неактивной кнопки Выход
        private Texture2D cursorExit2;          //Текстура активной кнопки Выход
        private Vector2 cursorPositionExit;     //Расположение кнопки Выход

        /// <summary>
        /// начальные значения
        /// </summary>
        public Menu()
        {
            menuPosition = new Vector2(0, 0);
            cursorPositionGame = new Vector2(23, 295);
            cursorPositionOption = new Vector2(142, 359);
            cursorPositionRecord = new Vector2(229, 424);
            cursorPositionAbout = new Vector2(314, 497);
            cursorPositionHTP = new Vector2(108, 528);
            cursorPositionExit = new Vector2(304, 577);
        }

        /// <summary>
        /// Загрузка текстур меню 
        /// </summary>
        /// <param name="content"></param>
        public void Load(ContentManager content)
        {
            menuTexture = content.Load<Texture2D>("Content\\Textures\\menu");       //Текстура фон
            cursorGame = content.Load<Texture2D>("Content\\Textures\\new1");        //Текстура неактивной кнопки Новая игра
            cursorGame2 = content.Load<Texture2D>("Content\\Textures\\new2");       //Текстура активной кнопки Новая игра
            cursorOption = content.Load<Texture2D>("Content\\Textures\\opt1");      //Текстура неактивной кнопки Опции
            cursorOption2 = content.Load<Texture2D>("Content\\Textures\\opt2");     //Текстура активной кнопки Опции
            cursorRecord = content.Load<Texture2D>("Content\\Textures\\records1");  //Текстура неактивной кнопки Рекорды
            cursorRecord2 = content.Load<Texture2D>("Content\\Textures\\records2"); //Текстура активной кнопки Рекорды
            cursorAbout = content.Load<Texture2D>("Content\\Textures\\about1");     //Текстура неактивной кнопки О...
            cursorAbout2 = content.Load<Texture2D>("Content\\Textures\\about2");    //Текстура активной кнопки О...
            cursorHTP = content.Load<Texture2D>("Content\\Textures\\htp1");         //Текстура неактивной кнопки Как играть
            cursorHTP2 = content.Load<Texture2D>("Content\\Textures\\htp2");        //Текстура активной кнопки Как играть
            cursorExit = content.Load<Texture2D>("Content\\Textures\\exit1");       //Текстура неактивной кнопки Выход
            cursorExit2 = content.Load<Texture2D>("Content\\Textures\\exit2");      //Текстура активной кнопки Выход
        }

        /// <summary>
        /// Отрисовка меню
        /// </summary>
        /// <param name="spriteBatch"></param>
        /// <param name="state"></param>
        public void DrawMenu(SpriteBatch spriteBatch, int state)
        {
            spriteBatch.Draw(menuTexture, menuPosition, Color.White);           //Отрисовка заднего фона
            switch (state)                  //Если активна какая то кнопка, то рисуем её активной, а остальные неактивными
            {
                case 1:
                spriteBatch.Draw(cursorGame2, cursorPositionGame, Color.White);
                spriteBatch.Draw(cursorOption, cursorPositionOption, Color.White);
                spriteBatch.Draw(cursorExit, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorRecord, cursorPositionRecord, Color.White);
                spriteBatch.Draw(cursorAbout, cursorPositionAbout, Color.White);
                spriteBatch.Draw(cursorHTP, cursorPositionHTP, Color.White);
                break;
                case 2:
                spriteBatch.Draw(cursorGame, cursorPositionGame, Color.White);
                spriteBatch.Draw(cursorOption, cursorPositionOption, Color.White);
                spriteBatch.Draw(cursorExit2, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorRecord, cursorPositionRecord, Color.White);
                spriteBatch.Draw(cursorAbout, cursorPositionAbout, Color.White);
                spriteBatch.Draw(cursorHTP, cursorPositionHTP, Color.White);
                break;
                case 3:
                spriteBatch.Draw(cursorGame, cursorPositionGame, Color.White);
                spriteBatch.Draw(cursorOption2, cursorPositionOption, Color.White);
                spriteBatch.Draw(cursorExit, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorRecord, cursorPositionRecord, Color.White);
                spriteBatch.Draw(cursorAbout, cursorPositionAbout, Color.White);
                spriteBatch.Draw(cursorHTP, cursorPositionHTP, Color.White);
                break;
                case 4:
                spriteBatch.Draw(cursorGame, cursorPositionGame, Color.White);
                spriteBatch.Draw(cursorOption, cursorPositionOption, Color.White);
                spriteBatch.Draw(cursorExit, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorRecord2, cursorPositionRecord, Color.White);
                spriteBatch.Draw(cursorAbout, cursorPositionAbout, Color.White);
                spriteBatch.Draw(cursorHTP, cursorPositionHTP, Color.White);
                break;
                case 5:
                spriteBatch.Draw(cursorGame, cursorPositionGame, Color.White);
                spriteBatch.Draw(cursorOption, cursorPositionOption, Color.White);
                spriteBatch.Draw(cursorExit, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorRecord, cursorPositionRecord, Color.White);
                spriteBatch.Draw(cursorAbout2, cursorPositionAbout, Color.White);
                spriteBatch.Draw(cursorHTP, cursorPositionHTP, Color.White);
                break;
                case 6:
                spriteBatch.Draw(cursorGame, cursorPositionGame, Color.White);
                spriteBatch.Draw(cursorOption, cursorPositionOption, Color.White);
                spriteBatch.Draw(cursorExit, cursorPositionExit, Color.White);
                spriteBatch.Draw(cursorRecord, cursorPositionRecord, Color.White);
                spriteBatch.Draw(cursorAbout, cursorPositionAbout, Color.White);
                spriteBatch.Draw(cursorHTP2, cursorPositionHTP, Color.White);
                break;
            }
        }
    }
}
