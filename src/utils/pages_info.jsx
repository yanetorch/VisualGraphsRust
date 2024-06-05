import Task1 from "../tasks/Task1";
import Task2 from "../tasks/Task2";
import Task3 from "../tasks/Task3";
import Task4 from "../tasks/Task4";
import Task5 from "../tasks/Task5";
import Task6 from "../tasks/Task6";
import Task7 from "../tasks/Task7";
import Task8 from "../tasks/Task8";
import Task9 from "../tasks/Task9";
import Task10 from "../tasks/Task10";
import Task11 from "../tasks/Task11";
import Task12 from "../tasks/Task12";

const pages_info = {

    "1" : {
        title: "Task1",
        component: Task1,
        description: "На вход — число n, на выход — булева функция от n аргументов.",
    },
    "2" : {
        title: "Task2",
        component: Task2,
        description: "На вход — вектор функции, 0 или 1, номер аргумента, на выход — соответствующая остаточная.",
    },
    "3" : {
        title: "Task3",
        component: Task3,
        description: "На вход — два вектора (это нулевая и единичная остаточные функции по некоторому аргументу), номер аргумента, на выход — вектор функции.",
    },
    "4" : {
        title: "Task4",
        component: Task4,
        description: "Игра. Узнать имя функции от 2-х аргументов. Система предлагает вектор функции, пользователь выбирает «имя» (одно из 16).",
    },
    "5" : {
        title: "Task5",
        component: Task5,
        description: " Игра. Существенные и фиктивные переменные. Система предлагает вектор функции. Пользователь выбирает существенные и фиктивные переменные.",
    },
    "6" : {
        title: "Task6",
        component: Task6,
        description: "Игра. ДНФ. Система предлагает вектор функции. Пользователь вводит ДНФ. Система определяет правильно или нет введена ДНФ.",
    },
    "7" : {
        title: "Task7",
        component: Task7,
        description: "Игра. КНФ. Система предлагает вектор функции. Пользователь вводит КНФ. Система определяет правильно или нет введена КНФ.",
    },
    "8" : {
        title: "Task8",
        component: Task8,
        description: "Пользователь вводит вектор функции. Система строит СДНФ.",
    },
    "9" : {
        title: "Task9",
        component: Task9,
        description: "Пользователь вводит вектор функции. Система строит СКНФ.",
    },
    "10" : {
        title: "Task10",
        component: Task10,
        description: "Игра. Предполные классы б.ф. \
        Система предлагает вектор функции. Пользователь должен выбрать предполные классы, которым эта функция принадлежит.\
        Система определяет правильно выбраны классы или нет.",
    },
    "11" : {
        title: "Task11",
        component: Task11,
        description: "Игра. Полные системы б.ф. Система предлагает набор векторов функций.\
        Пользователь определяет полным или нет является набор функций. Если система б.ф. неполна, то пользователь\
        должен указать замкнутый класс, которому набор функций принадлежит.",
    },
    "12" : {
        title: "Task12",
        component: Task12,
        description: "Построение ДНФ с помощью метода Куайна — Мак-Класки.",
    },

}

export default pages_info;