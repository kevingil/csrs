"""Author the default knife, arm poses and HUD art. Run with Blender 4.2.

The knife mesh is original geometry. Arms and portraits derive from the existing
character assets and retain their provenance. Source GLBs are never overwritten.
"""

import math
import sys
from pathlib import Path

import bpy  # ty: ignore[unresolved-import]
from mathutils import Matrix, Vector  # ty: ignore[unresolved-import]

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
from export_assets import clean, export, import_rig  # noqa: E402
from inspect_assets import apply_character_materials, read_glb, write_glb  # noqa: E402
from export_menu import camera, render  # noqa: E402

OUT = ROOT / "assets/generated"
UI = OUT / "ui"
CLIPS = {"idle_knife": 30, "draw_knife": 11, "slash_knife": 17}


def material(name, color, metal=0.0):
    result = bpy.data.materials.new(name)
    result.diffuse_color = (*color, 1)
    result.use_nodes = True
    shader = result.node_tree.nodes.get("Principled BSDF")
    shader.inputs["Base Color"].default_value = (*color, 1)
    shader.inputs["Metallic"].default_value = metal
    shader.inputs["Roughness"].default_value = 0.36 if metal else 0.75
    return result


def knife_mesh():
    grip = bpy.data.objects.new("KnifeGrip", None)
    bpy.context.collection.objects.link(grip)
    steel = material("Knife satin steel", (0.32, 0.38, 0.42), 0.8)
    rubber = material("Knife textured grip", (0.028, 0.032, 0.030))
    # Local +Y is the blade axis. The handle center sits in the palm.
    outline = [(-0.026, 0.08), (0.025, 0.08), (0.025, 0.29), (0, 0.40), (-0.025, 0.31)]
    verts = [(x, y, z) for z in [-0.003, 0.003] for x, y in outline]
    faces = [(4, 3, 2, 1, 0), (5, 6, 7, 8, 9)]
    faces += [(i, (i + 1) % 5, (i + 1) % 5 + 5, i + 5) for i in range(5)]
    mesh = bpy.data.meshes.new("Default knife blade")
    mesh.from_pydata(verts, [], faces)
    blade = bpy.data.objects.new("KnifeBlade", mesh)
    bpy.context.collection.objects.link(blade)
    blade.data.materials.append(steel)
    objects = [blade]
    for name, location, scale, mat in [
        ("KnifeHandle", (0, 0, 0), (0.019, 0.072, 0.017), rubber),
        ("KnifeGuard", (0, 0.077, 0), (0.043, 0.006, 0.014), steel),
        ("KnifePommel", (0, -0.071, 0), (0.021, 0.005, 0.019), steel),
    ]:
        bpy.ops.mesh.primitive_cube_add(size=2, location=location)
        obj = bpy.context.object
        obj.name = name
        obj.scale = scale
        bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)
        bevel = obj.modifiers.new("Soft edges", "BEVEL")
        bevel.width = 0.003
        bevel.segments = 2
        bpy.ops.object.modifier_apply(modifier=bevel.name)
        obj.data.materials.append(mat)
        objects.append(obj)
    for obj in objects:
        obj.parent = grip
    return grip, objects


def reach(rig, side, world_target, bend_direction):
    """Analytic two-bone IK with an authored elbow pole and retained wrist pose."""
    upper, lower, hand = [rig.pose.bones[f"mixamorig:{side}{part}"] for part in ["Arm", "ForeArm", "Hand"]]
    shoulder, elbow, wrist = upper.head.copy(), lower.head.copy(), hand.head.copy()
    wrist_rotation = hand.matrix.to_quaternion()
    target = rig.matrix_world.inverted() @ Vector(world_target)
    length1, length2 = (elbow - shoulder).length, (wrist - elbow).length
    axis = (target - shoulder).normalized()
    distance = min(max((target - shoulder).length, abs(length1 - length2) + 0.001), length1 + length2 - 0.001)
    along = (length1**2 - length2**2 + distance**2) / (2 * distance)
    pole = rig.matrix_world.inverted().to_3x3() @ Vector(bend_direction)
    bend = (pole - axis * pole.dot(axis)).normalized()
    goal_elbow = shoulder + axis * along + bend * math.sqrt(max(0, length1**2 - along**2))
    rotation = (elbow - shoulder).rotation_difference(goal_elbow - shoulder) @ upper.matrix.to_quaternion()
    upper.matrix = Matrix.LocRotScale(shoulder, rotation, upper.matrix.to_scale())
    bpy.context.view_layer.update()
    rotation = (hand.head - lower.head).rotation_difference(target - lower.head) @ lower.matrix.to_quaternion()
    lower.matrix = Matrix.LocRotScale(lower.head, rotation, lower.matrix.to_scale())
    bpy.context.view_layer.update()
    hand.matrix = Matrix.LocRotScale(hand.head, wrist_rotation, hand.matrix.to_scale())
    bpy.context.view_layer.update()


def author_actions(rig):
    for track in rig.animation_data.nla_tracks:
        track.mute = True
    rig.animation_data.action = bpy.data.actions["idle_rifle_Armature"]
    bpy.context.scene.frame_set(0)
    bpy.context.view_layer.update()
    base = {bone.name: bone.matrix_basis.copy() for bone in rig.pose.bones}
    for track in list(rig.animation_data.nla_tracks):
        rig.animation_data.nla_tracks.remove(track)
    actions = {}
    for name, end in CLIPS.items():
        action = bpy.data.actions.new(name + "_pose")
        rig.animation_data.action = action
        for frame in range(end + 1):
            bpy.context.scene.frame_set(frame)
            for bone in rig.pose.bones:
                bone.matrix_basis = base[bone.name]
            bpy.context.view_layer.update()
            t = frame / end
            # A single sweep reaches contact at 0.15 s, then returns to idle.
            sweep = math.sin(min(t / (0.15 / 0.55), 1) * math.pi / 2) if t < 0.15 / 0.55 else (1 - (t - 0.15 / 0.55) / (1 - 0.15 / 0.55)) ** 2
            sweep = sweep if name == "slash_knife" else 0
            draw = (1 - t) ** 2 if name == "draw_knife" else 0
            reach(rig, "Right", (-0.24 + 0.25 * sweep, -0.36 - 0.12 * sweep, 1.32 - 0.22 * draw), (-1, 0.3, -1))
            reach(rig, "Left", (0.24, -0.29, 1.25 - 0.12 * draw), (1, 0.3, -1))
            for bone in rig.pose.bones:
                bone.rotation_mode = "QUATERNION"
                bone.keyframe_insert("location", frame=frame, group=bone.name)
                bone.keyframe_insert("rotation_quaternion", frame=frame, group=bone.name)
                bone.keyframe_insert("scale", frame=frame, group=bone.name)
        actions[name] = action
    rig.animation_data.action = None
    for name, action in actions.items():
        track = rig.animation_data.nla_tracks.new()
        track.name = name
        track.strips.new(name, 0, action)
        track.mute = True
    rig.animation_data.action = actions["idle_knife"]
    bpy.context.scene.frame_set(0)
    bpy.context.view_layer.update()
    return actions


def export_view():
    clean()
    rig, imported = import_rig(OUT / "ak_view.glb")
    # Only the existing skinned arms are retained from the AK scene.
    arms = [o for o in imported if o.type == "MESH" and any(m.type == "ARMATURE" for m in o.modifiers)]
    for obj in list(imported):
        if obj != rig and obj not in arms:
            bpy.data.objects.remove(obj, do_unlink=True)
    author_actions(rig)
    grip, meshes = knife_mesh()
    palm = sum((rig.matrix_world @ rig.pose.bones[f"mixamorig:RightHandMiddle{i}"].head for i in [1, 2, 3]), Vector()) / 3
    grip.parent = rig
    grip.parent_type = "BONE"
    grip.parent_bone = "mixamorig:RightHand"
    grip.matrix_world = Matrix.LocRotScale(palm, Vector((0.85, -0.45, 0.23)).to_track_quat("Y", "Z"), Vector((1, 1, 1)))
    bpy.context.view_layer.update()
    rig.animation_data.action = None
    for track in rig.animation_data.nla_tracks:
        track.mute = False
    bpy.ops.wm.save_as_mainfile(filepath=str(ROOT / "assets/models/weapons/default_knife.blend"))
    export(OUT / "knife_view.glb", [rig, *arms, grip, *meshes])
    apply_character_materials(OUT / "knife_view.glb")
    view, _ = read_glb(OUT / "knife_view.glb")
    socket = next(n for n in view["nodes"] if n.get("name") == "KnifeGrip")
    grip.parent = None
    grip.matrix_world = Matrix.Identity(4)
    export(OUT / "knife_world.glb", [grip, *meshes])
    world, tail = read_glb(OUT / "knife_world.glb")
    node = next(n for n in world["nodes"] if n.get("name") == "KnifeGrip")
    for key in ["translation", "rotation", "scale"]:
        if key in socket:
            node[key] = socket[key]
    write_glb(OUT / "knife_world.glb", world, tail)


def export_character_poses(name):
    clean()
    rig, _ = import_rig(OUT / f"{name}.glb")
    author_actions(rig)
    rig.animation_data.action = None
    for track in rig.animation_data.nla_tracks:
        track.mute = False
    export(OUT / f"knife_pose_{name}.glb", [rig])


def hud_art():
    UI.mkdir(exist_ok=True)
    clean()
    grip, meshes = knife_mesh()
    white = material("HUD white", (1, 1, 1))
    shader = white.node_tree.nodes.get("Principled BSDF")
    shader.inputs["Emission Color"].default_value = (1, 1, 1, 1)
    shader.inputs["Emission Strength"].default_value = 1
    for obj in meshes:
        obj.data.materials.clear()
        obj.data.materials.append(white)
    cam = camera((0, 0.15, 2), (0, 0.15, 0))
    cam.rotation_euler.z = math.pi / 2
    cam.data.type = "ORTHO"
    cam.data.ortho_scale = 0.58
    bpy.context.scene.render.film_transparent = True
    render(UI / "knife.png", 384, 128)
    for name in ["attacker", "defender"]:
        clean()
        rig, _ = import_rig(OUT / f"{name}.glb")
        rig.animation_data.action = bpy.data.actions["idle_rifle_Armature"]
        bpy.context.scene.frame_set(0)
        bpy.context.view_layer.update()
        head = rig.matrix_world @ rig.pose.bones["mixamorig:Head"].head
        camera(head + Vector((0, -1.1, 0.05)), head + Vector((0, 0, -0.04)), 60)
        bpy.ops.object.light_add(type="AREA", location=head + Vector((-0.7, -1, 1)))
        bpy.context.object.data.energy = 140
        bpy.context.object.data.shape = "DISK"
        bpy.context.object.data.size = 1.5
        bpy.context.object.rotation_euler = (Vector(head) - bpy.context.object.location).to_track_quat("-Z", "Y").to_euler()
        bpy.context.scene.world = bpy.data.worlds.new("Portrait world")
        bpy.context.scene.world.color = (0.15, 0.15, 0.15)
        bpy.context.scene.render.film_transparent = True
        render(UI / f"{name}_portrait.png", 160, 160)


def main():
    export_view()
    for name in ["attacker", "defender"]:
        export_character_poses(name)
    hud_art()


if __name__ == "__main__":
    main()
