//
//  BPXEditTests.swift
//  BPXEditTests
//
//  Created by Yuri Edward on 1/6/26.
//

import Testing
import BPXEdit

struct BPXEditTests {

    @Test func testBasic() async throws {
        let data = NSDataStream(Data())
        let container = BPXStream(from: data).create();
        try container.save()
        assert(!data.data().isEmpty)
        let data2 = NSDataStream(data.data())
        let container2 = try BPXStream(from: data2).open();
        assert(container2.mainHeader.file_size > 0)
    }

    @Test func testTable() async throws {
        let data = NSDataStream(Data())
        let container = BPXStream(from: data).create();
        let strings = container.createStrings();
        let table = try container.createTable(strings.section, name: "Test");
        let a = try table.addColumn("A", type: .uint8, len: 1);
        let b = try table.addColumn("B", type: .float, len: 1);
        let c = try table.addColumn("C", type: .string, len: 8);
        try table.save();
        let row = table.newRow();
        row[a].setInt64(0xFF);
        row[b].setDouble(0.42);
        row[c].setString("test");
        try table.append(row);
        row[c].setString("value");
        try table.append(row);
        row[c].setString("another value");
        try table.append(row);
        try container.save();

        let data1 = NSDataStream(data.data())
        let container1 = try BPXStream(from: data1).open();
        let strings1 = container1.sections.first(where: { $0.header.type == 0xFF });
        let table1 = try container1.sections.first(where: { $0.header.type == 0xFD })!.openTable(strings1!);
        let a1 = try table1.column(forName: "A");
        let b1 = try table1.column(forName: "B");
        let c1 = try table1.column(forName: "C");
        assert(table1.rowCount == 3);
        var row1 = try table1.read(0);
        assert(!row1.isFree());
        assert(0xFF == row1[a1].toInt64());
        assert(abs(0.42 - row1[b1].toDouble()) < 0.001);
        assert(row1[c1].s == "test");
        row1 = try table1.read(1);
        assert(!row1.isFree());
        assert(0xFF == row1[a1].toInt64());
        assert(abs(0.42 - row1[b1].toDouble()) < 0.001);
        assert(row1[c1].s == "value");
        row1 = try table1.read(2);
        assert(!row1.isFree());
        assert(0xFF == row1[a1].toInt64());
        assert(abs(0.42 - row1[b1].toDouble()) < 0.001);
        assert(row1[c1].s == "another ");
    }

}
