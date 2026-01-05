"""
Blender script to clean up a Mixamo skin model's hierarchy.

This script flattens the hierarchy so the armature is at the root level,
making it compatible with separate animation files that have a simple
"Armature -> bones" structure.

The animation file has: Armature -> mixamorig:Hips -> ...
This script makes the skin model match that structure.

Usage:
1. Open Blender
2. File -> Import -> glTF 2.0 -> select attacker_default_skin.glb
3. Open Scripting workspace
4. Open this script and run it
5. Export: File -> Export -> glTF 2.0 (.glb)
"""

import bpy


def clean_hierarchy():
    """
    Clean up the skin model hierarchy:
    1. Find the armature
    2. Unparent it from any wrapper objects
    3. Rename it to "Armature" (to match animation file)
    4. Delete unnecessary wrapper objects
    5. Place armature at origin
    """
    
    # Find all armatures
    armatures = [obj for obj in bpy.data.objects if obj.type == 'ARMATURE']
    
    if not armatures:
        print("ERROR: No armature found in scene!")
        return False
    
    armature = armatures[0]
    print(f"Found armature: {armature.name}")
    
    # Find all mesh objects (they should be children of or use the armature)
    meshes = [obj for obj in bpy.data.objects if obj.type == 'MESH']
    print(f"Found {len(meshes)} mesh objects")
    
    # Store mesh-armature relationships before unparenting
    mesh_modifiers = {}
    for mesh in meshes:
        for mod in mesh.modifiers:
            if mod.type == 'ARMATURE' and mod.object == armature:
                mesh_modifiers[mesh.name] = mod.name
                break
    
    # Select all objects first
    bpy.ops.object.select_all(action='SELECT')
    
    # Store the original armature parent
    original_parent = armature.parent
    
    # Unparent the armature while keeping its transform
    bpy.context.view_layer.objects.active = armature
    bpy.ops.object.select_all(action='DESELECT')
    armature.select_set(True)
    
    if armature.parent:
        # Store world matrix before unparenting
        world_matrix = armature.matrix_world.copy()
        armature.parent = None
        armature.matrix_world = world_matrix
        print(f"Unparented armature from {original_parent.name if original_parent else 'None'}")
    
    # Rename armature to "Armature" (standard name expected by animations)
    old_name = armature.name
    armature.name = "Armature"
    print(f"Renamed armature from '{old_name}' to 'Armature'")
    
    # Also rename the armature data
    if armature.data:
        armature.data.name = "Armature"
    
    # Move meshes to be children of the armature (or at root level)
    for mesh in meshes:
        if mesh.parent and mesh.parent != armature:
            # Store world transform
            world_matrix = mesh.matrix_world.copy()
            
            # Unparent
            mesh.parent = None
            mesh.matrix_world = world_matrix
            
            print(f"Unparented mesh '{mesh.name}'")
    
    # Delete empty wrapper objects
    objects_to_delete = []
    for obj in bpy.data.objects:
        if obj.type == 'EMPTY' and obj not in [armature] + meshes:
            # Check if it's a wrapper with no important children
            if not any(child.type in ['ARMATURE', 'MESH'] for child in obj.children):
                objects_to_delete.append(obj)
    
    # Also delete the Sketchfab wrapper if it exists
    for obj in bpy.data.objects:
        if 'Sketchfab' in obj.name or 'fbx' in obj.name.lower():
            if obj.type == 'EMPTY':
                objects_to_delete.append(obj)
    
    # Delete unnecessary objects
    bpy.ops.object.select_all(action='DESELECT')
    for obj in objects_to_delete:
        if obj.name in bpy.data.objects:
            print(f"Deleting empty wrapper: {obj.name}")
            bpy.data.objects.remove(obj, do_unlink=True)
    
    # Reset armature to origin
    armature.location = (0, 0, 0)
    armature.rotation_euler = (0, 0, 0)
    
    print("\n=== Hierarchy cleaned! ===")
    print(f"Armature is now named: {armature.name}")
    print(f"Root bone: {armature.data.bones[0].name if armature.data.bones else 'None'}")
    
    # Print final structure
    print("\nFinal scene hierarchy:")
    for obj in bpy.data.objects:
        if obj.parent is None:
            print_hierarchy(obj, 0)
    
    return True


def print_hierarchy(obj, depth):
    """Print the object hierarchy"""
    indent = "  " * depth
    print(f"{indent}- {obj.name} ({obj.type})")
    for child in obj.children:
        print_hierarchy(child, depth + 1)


def rename_root_bone_if_needed():
    """
    Ensure the root bone is named correctly.
    If it's _rootJoint, that's fine - but we need the armature 
    object itself to be named "Armature".
    """
    for armature in [obj for obj in bpy.data.objects if obj.type == 'ARMATURE']:
        bones = armature.data.bones
        if bones:
            root_bones = [b for b in bones if b.parent is None]
            for bone in root_bones:
                print(f"Root bone: {bone.name}")


if __name__ == "__main__":
    print("\n" + "="*50)
    print("CLEANING SKIN MODEL HIERARCHY")
    print("="*50 + "\n")
    
    success = clean_hierarchy()
    
    if success:
        rename_root_bone_if_needed()
        print("\n" + "="*50)
        print("SUCCESS! Now export the model:")
        print("File -> Export -> glTF 2.0 (.glb)")
        print("Overwrite: assets/models/skins/attacker_default_skin.glb")
        print("="*50)
    else:
        print("\nFailed to clean hierarchy. Check errors above.")

