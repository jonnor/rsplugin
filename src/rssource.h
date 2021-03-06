/* Copyright (C) 2016 Sebastian Dröge <sebastian@centricular.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef __GST_RS_SRC_H__
#define __GST_RS_SRC_H__

#include <gst/gst.h>
#include <gst/base/base.h>

G_BEGIN_DECLS

#define GST_RS_SRC(obj) \
  ((GstRsSrc *)obj)
#define GST_RS_SRC_CLASS(klass) \
  ((GstRsSrcKlass *)klass)

typedef struct _GstRsSrc GstRsSrc;
typedef struct _GstRsSrcClass GstRsSrcClass;

struct _GstRsSrc {
  GstPushSrc element;

  gpointer instance;
};

struct _GstRsSrcClass {
  GstPushSrcClass parent_class;
};

G_GNUC_INTERNAL gboolean gst_rs_source_plugin_init (GstPlugin * plugin);

G_END_DECLS

#endif /* __GST_RS_SRC_H__ */
