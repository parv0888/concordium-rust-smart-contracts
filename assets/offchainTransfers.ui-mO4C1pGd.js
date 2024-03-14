import{offchainTransfers as t,initRequestSchemaBase64 as a,addSettlementRequestSchemaBase64 as n,getSettlementRequestSchemaBase64 as r,getSettlementResponseSchemaBase64 as m,settledBalanceOfRequestSchemaBase64 as o,settledBalanceOfResponseSchemaBase64 as l,withdrawRequestSchemaBase64 as c}from"./offchainTransfers-CrsUh7Wz.js";import{G as p,b as s,a as i}from"./GenericContractUI-v_xjRjC2.js";import"./GenericContract-C24tSQIY.js";import"./index-BD4uXlNQ.js";const d={type:"object",title:"Init Request",properties:{validator:{type:"string",title:"Validator"},judge:{type:"string",title:"Judge"},time_to_finality:{type:"string",title:"Time To Finality"},settlement_limit:{type:"integer",minimum:0,maximum:4294967295,title:"Settlement Limit"}}},S={type:"object",title:"Add Settlement Request",properties:{send_transfers:{type:"array",items:{type:"object",title:"",properties:{address:{type:"string",title:"Address"},amount:{type:"string",title:"Amount"}}},title:"Send Transfers"},receive_transfers:{type:"array",items:{type:"object",title:"",properties:{address:{type:"string",title:"Address"},amount:{type:"string",title:"Amount"}}},title:"Receive Transfers"},meta_data:{type:"array",items:{type:"integer",minimum:0,maximum:255,title:""},title:"Meta Data"}}},u={type:"integer",minimum:0,title:"Get Settlement Request"},y={type:"object",title:"Get Settlement Response",properties:{id:{type:"integer",minimum:0,title:"Id"},transfer:{type:"object",title:"Transfer",properties:{send_transfers:{type:"array",items:{type:"object",title:"",properties:{address:{type:"string",title:"Address"},amount:{type:"string",title:"Amount"}}},title:"Send Transfers"},receive_transfers:{type:"array",items:{type:"object",title:"",properties:{address:{type:"string",title:"Address"},amount:{type:"string",title:"Amount"}}},title:"Receive Transfers"},meta_data:{type:"array",items:{type:"integer",minimum:0,maximum:255,title:""},title:"Meta Data"}}},finality_time:{type:"string",format:"date-time",title:"Finality Time"}}},h={type:"string",title:"Settled Balance Of Request"},g={type:"string",title:"Settled Balance Of Response"},f={type:"string",title:"Withdraw Request"},T=e=>p({onContractInitialized:e.onInitialize,uiSchema:e.uiSchema,uiWidgets:e.uiWidgets,method:t.init,requestJsonSchema:d,requestSchemaBase64:a}),_={addSettlement:e=>s({...e,method:t.addSettlement,requestJsonSchema:S,requestSchemaBase64:n}),getSettlement:e=>i({...e,method:t.getSettlement,requestJsonSchema:u,requestSchemaBase64:r,responseJsonSchema:y,responseSchemaBase64:m}),settledBalanceOf:e=>i({...e,method:t.settledBalanceOf,requestJsonSchema:h,requestSchemaBase64:o,responseJsonSchema:g,responseSchemaBase64:l}),withdraw:e=>s({...e,method:t.withdraw,requestJsonSchema:f,requestSchemaBase64:c})};export{_ as ENTRYPOINTS_UI,S as addSettlementRequestJsonSchema,u as getSettlementRequestJsonSchema,y as getSettlementResponseJsonSchema,T as init,d as initRequestJsonSchema,h as settledBalanceOfRequestJsonSchema,g as settledBalanceOfResponseJsonSchema,f as withdrawRequestJsonSchema};
