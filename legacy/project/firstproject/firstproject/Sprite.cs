using System;
using System.Collections.Generic;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Audio;
using Microsoft.Xna.Framework.Content;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;
using Microsoft.Xna.Framework.Storage;

namespace firstproject
{

    public class Sprite
    {
        private int framecount;                         //количество кадров в анимации
        private double timeframe;                       //время кадра
        private int frame;                              //номер кадра
        private double totalelapsed;                    //время между переключением кадров

        public Texture2D spriteTexture;                 //Текстура объекта
        public Vector2 spritePosition;                  //Позиция объекта
        public Vector2 aSprite = new Vector2(0, 1);     //Ускорение объекта
        public Vector2 xSprite = new Vector2(0, 0);     //Скорость объекта

        public bool alive;                              //Активность объекта

        public Sprite()
        {
        }

        /// <summary>
        /// вычисление количества кадров и время кадра
        /// </summary>
        /// <param name="frameCount"></param>
        /// <param name="framesPerSec"></param>
        public Sprite(int frameCount, double framesPerSec)
        {
            framecount = frameCount;
            timeframe = (float)1 / framesPerSec;            //framesPerSec кол-во кадров в секунду
            frame = 0;
            totalelapsed = 0;
        }
        /// <summary>
        /// Обновление кадра
        /// </summary>
        /// <param name="elapsed"></param>
        public void UpdateFrame(double elapsed)
        {
            totalelapsed += elapsed;
            if (totalelapsed > timeframe)
            {
                frame++;
                frame = frame % (framecount);
                if (xSprite.Y > 15) frame = 0;   
                totalelapsed -= timeframe;
            }
        }
  
        /// <summary>
        /// Загрузка текстуры
        /// </summary>
        /// <param name="content"></param>
        /// <param name="stringTexture"></param>
        public void Load(ContentManager content, String stringTexture)
        {
            spriteTexture = content.Load<Texture2D>(stringTexture);
        }

       /// <summary>
       /// отрисовка текстуры неанимированного объекта
       /// </summary>
       /// <param name="spriteBatch"></param>
        public void DrawSprite(SpriteBatch spriteBatch)
        {
            spriteBatch.Draw(spriteTexture, spritePosition, Color.White);
        }

     
        /// <summary>
        /// отрисовка текстуры анимированного объекта
        /// </summary>
        /// <param name="spriteBatch"></param>
        public void DrawAnimationSprite(SpriteBatch spriteBatch)
        {
            int frameWidth = spriteTexture.Width / framecount ;         //размер кадра
            Rectangle rectangle = new Rectangle(frameWidth * frame, 0, frameWidth,spriteTexture.Height);        //показ части текстуры
            spriteBatch.Draw(spriteTexture, spritePosition, rectangle, Color.White);
        }
    }
}
