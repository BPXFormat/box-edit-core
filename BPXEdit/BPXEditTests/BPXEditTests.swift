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
        let data = NSDataStream(Data());
        let stream = BPXStream(from: data);
        let container = BPXContainer.create(stream);
        try container.save();
        assert(!data.data().isEmpty)
    }

}
