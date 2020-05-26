extends Node2D

onready var _server = get_node("../LogicController")

func _input(event: InputEvent):
	if event is InputEventKey:
		if event.pressed:
			match event.scancode:
				KEY_W:
					_server.try_move(Vector2(0, -1))
				KEY_S:
					_server.try_move(Vector2(0, 1))			
				KEY_A:
					_server.try_move(Vector2(-1, 0))			
				KEY_D:
					_server.try_move(Vector2(1, 0))			
