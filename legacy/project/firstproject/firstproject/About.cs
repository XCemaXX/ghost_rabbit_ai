using System;
using System.Collections.Generic;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Content;
using Microsoft.Xna.Framework.Graphics;

namespace firstproject
{
    class About
    {
        private Texture2D aboutTexture;         //Текстура для фона меню О...
        private Texture2D htpTexture;           //Текстура для фона меню Как играть
        private Vector2 aboutPosition;          //Положение текстуры фона
        private Texture2D cursorExit;           //Текстура кнопки Назад
        private Vector2 cursorPositionExit;     //Положение текстуры кнопки Назад

        /// <summary>
        /// начальные значения
        /// </summary>
        public About()
        {aboutPosition = new Vector2(0, 0);
        cursorPositionExit = new Vector2(204, 577);
        }

        /// <summary>
        /// загрузка текстур
        /// </summary>
        /// <param name="content"></param>
        public void Load(ContentManager content)
        {
            aboutTexture = content.Load<Texture2D>("Content\\Textures\\bg_about");      //текстура О...
            htpTexture = content.Load<Texture2D>("Content\\Textures\\bg_htp");          //текстура Как играть
            cursorExit = content.Load<Texture2D>("Content\\Textures\\back2");           //текстура Назад
        }

        /// <summary>
        /// отрисовка меню как играть или о...
        /// </summary>
        /// <param name="spriteBatch"></param>
        /// <param name="x"></param>
        public void DrawAbout(SpriteBatch spriteBatch, int x)
        {
            if (x==4) 
                spriteBatch.Draw(aboutTexture, aboutPosition, Color.White);     //отрисовка фона
            else
                spriteBatch.Draw(htpTexture, aboutPosition, Color.White);

            spriteBatch.Draw(cursorExit, cursorPositionExit, Color.White);      //отрисовка кнопки назад 
        }
    }
}
