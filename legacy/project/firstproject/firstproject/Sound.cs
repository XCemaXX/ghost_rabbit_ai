using System;
using System.Collections.Generic;
using System.Text;
using System.IO;
using Microsoft.Xna.Framework.Audio;

namespace firstproject
{
    public enum soundList           //структура данных, чтобы можно было обращаться к звукам "по имени"
    {
        dead,
        fall,
        hit,
        Start,
        thous,
    }

    public static class Sound
    {   
        public static AudioEngine engine;
        public static SoundBank soundbank;
        public static WaveBank wavebank;
        
        /// <summary>
        /// загрузка в игру звуковых данных
        /// </summary>
        public static void Initialize()    
        {
            engine = new AudioEngine("Content\\Sound\\Sound.xgs");
            soundbank = new SoundBank(engine, "Content\\Sound\\Sound Bank.xsb");
            wavebank = new WaveBank(engine, "Content\\Sound\\Wave Bank.xwb");
        }

        private static string[] cueNames = new string[]     //массив данных, чтобы можно было обращаться к звукам "по имени"
        {
            "dead",
            "fall",
            "hit",
            "Start",
            "thous",
        };

        /// <summary>
        /// включить звук
        /// </summary>
        /// <param name="sound"></param>
        public static void PlayCue(soundList sound)
        {
            soundbank.PlayCue(cueNames[(int)sound]);
        }

        /// <summary>
        /// обновление работы звукового движка
        /// </summary>
        public static void Update()
        {
            engine.Update();
        }

    }
}
