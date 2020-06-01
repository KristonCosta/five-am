extends Camera2D

func _input(event: InputEvent):
	var map = get_node("../MapController/TileMap")
	var cell_size = map.cell_size
	if event.is_action_pressed("zoom_in"):
		zoom *= 1.1
	elif event.is_action_pressed("zoom_out"):	
		zoom *= 0.9
	elif event is InputEventKey:
		if event.pressed:
			match event.scancode:
				KEY_UP:
					position.y -= 1 * cell_size.y 
				KEY_DOWN:
					position.y += 1 * cell_size.y 			
				KEY_LEFT:
					position.x -= 1 * cell_size.x 			
				KEY_RIGHT:
					position.x += 1 * cell_size.x 			
