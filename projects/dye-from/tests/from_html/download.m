(* ::Package:: *)

SetDirectory@NotebookDirectory[];
highlightjs = Import["https://highlightjs.org/static/demo/", "Text"];


Export["highlightjs.in.html", highlightjs, "Text"]
