```mermaid
---
title:enum
---
flowchart BT
	11@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_c\")]" }
	12@{ shape: stadium,label: "UnitVariant" }
	13@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_d\")]" }
	7@{ shape: stadium,label: "TupleVariant" }
	1@{ shape: hexagon,label: "EnumSample" }
	3@{ shape: lean-l,label: "< 'a , 'b , T >" }
	5@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_a\")]" }
	2@{ shape: lean-l,label: "< 'a , 'b , T : 'b >" }
	4@{ shape: lean-l,label: "where 'a : 'b ," }
	6@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_a\")]" }
	8@{ shape: subroutine,label: "#35; [cfg (feature = \"dummy_b\")]" }
	9@{ shape: subroutine,label: "#35; [doc = \"document\"]" }
	10@{ shape: stadium,label: "NamedVariant" }
	1 -- "variant" --> 7
	7 -- "attr" --> 8
	1 -- "impl" --> 2
	1 -- "variant" --> 10
	1 -- "type" --> 3
	7 -- "attr" --> 9
	1 -- "variant" --> 12
	12 -- "attr" --> 13
	1 -- "where" --> 4
	1 -- "attr" --> 6
	10 -- "attr" --> 11
	1 -- "attr" --> 5
```
