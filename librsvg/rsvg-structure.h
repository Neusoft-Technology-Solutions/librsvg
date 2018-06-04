/* -*- Mode: C; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/* vim: set sw=4 sts=4 expandtab: */
/*
   rsvg-structure.h: Rsvg's structual elements

   Copyright (C) 2000 Eazel, Inc.
   Copyright (C) 2002, 2003, 2004, 2005 Dom Lachowicz <cinamod@hotmail.com>
   Copyright (C) 2003, 2004, 2005 Caleb Moore <c.moore@student.unsw.edu.au>

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU Library General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Library General Public License for more details.

   You should have received a copy of the GNU Library General Public
   License along with this program; if not, write to the
   Free Software Foundation, Inc., 59 Temple Place - Suite 330,
   Boston, MA 02111-1307, USA.

   Authors: Raph Levien <raph@artofcode.com>, 
            Dom Lachowicz <cinamod@hotmail.com>, 
            Caleb Moore <c.moore@student.unsw.edu.au>
*/

#ifndef RSVG_STRUCTURE_H
#define RSVG_STRUCTURE_H

#include "rsvg-private.h"

G_BEGIN_DECLS 

/* Implemented in rust/src/load.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_load_new_node (const char *element_name, RsvgNode *parent, const char *id, gboolean *supports_class_attribute);

/* Implemented in rust/src/structure.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_group_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/link.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_link_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/structure.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_defs_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/structure.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_switch_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/structure.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_svg_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/structure.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_use_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/structure.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_symbol_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/image.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_image_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/structure.rs */
G_GNUC_INTERNAL
void rsvg_node_svg_get_size (RsvgNode *node, RsvgLength *out_width, RsvgLength *out_height);

/* Implemented in rust/src/structure.rs */
G_GNUC_INTERNAL
RsvgViewBox rsvg_node_svg_get_view_box (RsvgNode *node);

/* Implemented in rust/src/structure.rs */
G_GNUC_INTERNAL
void rsvg_node_svg_apply_atts (RsvgNode *node, RsvgHandle *handle);

/* Implemented in rust/src/text.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_text_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/text.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_tref_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/text.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_tspan_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/marker.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_marker_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/gradient.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_linear_gradient_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/gradient.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_radial_gradient_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/pattern.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_pattern_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/stop.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_stop_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/mask.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_mask_new (const char *element_name, RsvgNode *node, const char *id);

/* Implemented in rust/src/clip_path.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_clip_path_new (const char *element_name, RsvgNode *node, const char *id);

/* Implemented in rust/src/shapes.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_path_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/shapes.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_polygon_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/shapes.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_polyline_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/shapes.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_line_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/shapes.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_rect_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/shapes.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_circle_new (const char *element_name, RsvgNode *parent, const char *id);

/* Implemented in rust/src/shapes.rs */
G_GNUC_INTERNAL
RsvgNode *rsvg_node_ellipse_new (const char *element_name, RsvgNode *parent, const char *id);

G_END_DECLS

#endif                          /* RSVG_STRUCTURE_H */
