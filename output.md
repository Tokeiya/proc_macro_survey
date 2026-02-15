```mermaid
---
title:enum
---
flowchart BT
	8@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_b\")]" }
	10@{ shape: stadium,label: "NamedVariant" }
	13@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_d\")]" }
	11@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_c\")]" }
	12@{ shape: stadium,label: "UnitVariant" }
	9@{ shape: subroutine,label: "#35; [doc = \"document\"]" }
	2@{ shape: lean-l,label: "< 'a , 'b , T : 'b >" }
	7@{ shape: stadium,label: "TupleVariant" }
	3@{ shape: lean-l,label: "< 'a , 'b , T >" }
	1@{ shape: hexagon,label: "EnumSample" }
	4@{ shape: lean-l,label: "where 'a : 'b ," }
	5@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_a\")]" }
	6@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_a\")]" }
	1 -- "type" --> 3
	1 -- "attr" --> 6
	1 -- "variant" --> 10
	1 -- "attr" --> 5
	1 -- "variant" --> 7
	1 -- "where" --> 4
	7 -- "attr" --> 8
	7 -- "attr" --> 9
	1 -- "impl" --> 2
	10 -- "attr" --> 11
	1 -- "variant" --> 12
	12 -- "attr" --> 13
```
