#!/usr/bin/env node
// Authors: Joysusy & Violet Klaudia 💖
// Lylacore MCP Server — Exposes SDK as MCP tools for Claude Code

const { Server } = require('@modelcontextprotocol/sdk/server/index.js');
const { StdioServerTransport } = require('@modelcontextprotocol/sdk/server/stdio.js');
const {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} = require('@modelcontextprotocol/sdk/types.js');
const path = require('path');
const { loadSchema, validateMind, loadMind, loadMindsFromDirectory } = require('../sdk/mind-loader.js');

const server = new Server(
  {
    name: 'lylacore-ctx',
    version: '0.1.0',
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

/**
 * Tool: lylacore_validate_mind
 * Validates a Mind JSON object against the Mind Schema v1.
 */
server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: [
      {
        name: 'lylacore_validate_mind',
        description: 'Validates a Mind JSON object against the Mind Schema v1. Returns validation result with errors if invalid.',
        inputSchema: {
          type: 'object',
          properties: {
            mindJson: {
              type: 'string',
              description: 'JSON string of the Mind object to validate',
            },
          },
          required: ['mindJson'],
        },
      },
      {
        name: 'lylacore_load_mind',
        description: 'Loads and validates a Mind definition from a file path. Returns the Mind object with metadata.',
        inputSchema: {
          type: 'object',
          properties: {
            filePath: {
              type: 'string',
              description: 'Absolute path to the Mind JSON file',
            },
          },
          required: ['filePath'],
        },
      },
      {
        name: 'lylacore_list_minds',
        description: 'Lists all valid Mind definitions in a directory. Returns array of Mind objects.',
        inputSchema: {
          type: 'object',
          properties: {
            dirPath: {
              type: 'string',
              description: 'Absolute path to directory containing Mind JSON files',
            },
          },
          required: ['dirPath'],
        },
      },
    ],
  };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  try {
    switch (name) {
      case 'lylacore_validate_mind': {
        const mindData = JSON.parse(args.mindJson);
        const schema = loadSchema();
        const result = validateMind(mindData, schema);
        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify(result, null, 2),
            },
          ],
        };
      }

      case 'lylacore_load_mind': {
        const mind = loadMind(args.filePath);
        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                mind,
                metadata: {
                  source: mind._source,
                  loadedAt: mind._loadedAt,
                },
              }, null, 2),
            },
          ],
        };
      }

      case 'lylacore_list_minds': {
        const minds = loadMindsFromDirectory(args.dirPath);
        return {
          content: [
            {
              type: 'text',
              text: JSON.stringify({
                minds,
                count: minds.length,
              }, null, 2),
            },
          ],
        };
      }

      default:
        throw new Error(`Unknown tool: ${name}`);
    }
  } catch (error) {
    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify({
            error: error.message,
            stack: error.stack,
          }, null, 2),
        },
      ],
      isError: true,
    };
  }
});

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error('Lylacore MCP Server running on stdio');
}

main().catch((error) => {
  console.error('Fatal error:', error);
  process.exit(1);
});
