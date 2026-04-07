#!/usr/bin/env node
"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const fs = __importStar(require("fs"));
// Helper function to upload image to Imgur and get URL
async function uploadImageToImgur(filePath) {
    const fileBuffer = fs.readFileSync(filePath);
    const base64Image = fileBuffer.toString('base64');
    const response = await fetch('https://api.imgur.com/3/image', {
        method: 'POST',
        headers: {
            'Authorization': 'Client-ID 546c25a59c58ad7',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            image: base64Image,
            type: 'base64'
        })
    });
    if (!response.ok) {
        throw new Error(`Imgur upload failed: \${response.statusText}\`);
  }

  const data = await response.json() as { data: { link: string } };
  return data.data.link;
}

// Helper function to convert text to ProseMirror JSON format
function textToProseMirror(text: string): any {
  const paragraphs = text.split('\n\n').filter(p => p.trim());
  return {
    type: 'doc',
    content: paragraphs.map(para => ({
      type: 'paragraph',
      content: [{
        type: 'text',
        text: para.replace(/\n/g, ' ')
      }]
    }))
  };
}
        );
    }
}
